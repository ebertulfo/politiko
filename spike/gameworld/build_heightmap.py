#!/usr/bin/env python3
"""Prototype DEM builder for Politiko's gameworld map.

Pulls public-domain elevation from AWS Open Data "Terrain Tiles" (terrarium PNG,
no auth) across the Philippines bounding box, decodes to meters, stitches, crops
to the bbox, and emits:
  - heightmap_rg.png : 16-bit elevation split across R(hi)/G(lo) bytes (the mesh
                       source; survives Godot's 16->8 bit PNG downsample)
  - relief.png       : full-res natural-colour + hillshade texture (the map look)
  - hillshade.png    : small shaded-relief preview
  - meta.json        : min/max elevation, dims, scaling info for Godot

THROWAWAY SPIKE. Mercator distortion is left uncorrected (fine for a look-see).
"""
import json, math, io, sys, urllib.request, concurrent.futures as cf
from pathlib import Path
import numpy as np
from PIL import Image

# PH bounding box (from project memory / data-pipeline.md)
WEST, SOUTH, EAST, NORTH = 116.0, 4.5, 127.0, 21.5
ZOOM = 9                              # ~425 m/px; bump for more detail
TILE = 256
BASE = "https://s3.amazonaws.com/elevation-tiles-prod/terrarium/{z}/{x}/{y}.png"
OUT = Path(__file__).parent / "data"
OUT.mkdir(parents=True, exist_ok=True)


def lon2x(lon, z):
    return (lon + 180.0) / 360.0 * (2 ** z)


def lat2y(lat, z):
    r = math.radians(lat)
    return (1.0 - math.asinh(math.tan(r)) / math.pi) / 2.0 * (2 ** z)


def fetch(z, x, y):
    url = BASE.format(z=z, x=x, y=y)
    for attempt in range(4):
        try:
            req = urllib.request.Request(url, headers={"User-Agent": "politiko-spike"})
            data = urllib.request.urlopen(req, timeout=40).read()
            return (x, y, np.asarray(Image.open(io.BytesIO(data)).convert("RGB"), dtype=np.float64))
        except Exception as e:
            if attempt == 3:
                print(f"  tile {z}/{x}/{y} failed: {e}", file=sys.stderr)
                return (x, y, np.full((TILE, TILE, 3), [128, 0, 0], dtype=np.float64))  # ->elev 0


def colour_ramp(t):
    """Natural relief palette over t in [0,1] (t = sqrt of normalized height)."""
    xp = np.array([0.00, 0.05, 0.15, 0.32, 0.52, 0.75, 1.00])
    rs = np.array([0.93, 0.55, 0.38, 0.70, 0.62, 0.58, 0.97])
    gs = np.array([0.89, 0.72, 0.60, 0.68, 0.50, 0.55, 0.97])
    bs = np.array([0.69, 0.42, 0.30, 0.42, 0.34, 0.53, 0.98])
    return (np.interp(t, xp, rs), np.interp(t, xp, gs), np.interp(t, xp, bs))


def main():
    x0, x1 = int(math.floor(lon2x(WEST, ZOOM))), int(math.floor(lon2x(EAST, ZOOM)))
    y0, y1 = int(math.floor(lat2y(NORTH, ZOOM))), int(math.floor(lat2y(SOUTH, ZOOM)))
    nx, ny = x1 - x0 + 1, y1 - y0 + 1
    print(f"zoom {ZOOM}: x {x0}..{x1} ({nx}), y {y0}..{y1} ({ny}) = {nx*ny} tiles")

    canvas = np.zeros((ny * TILE, nx * TILE, 3), dtype=np.float64)
    tiles = [(ZOOM, x, y) for x in range(x0, x1 + 1) for y in range(y0, y1 + 1)]
    done = 0
    with cf.ThreadPoolExecutor(max_workers=24) as ex:
        for x, y, rgb in ex.map(lambda t: fetch(*t), tiles):
            cx, cy = (x - x0) * TILE, (y - y0) * TILE
            canvas[cy:cy + TILE, cx:cx + TILE] = rgb
            done += 1
            if done % 50 == 0 or done == len(tiles):
                print(f"  {done}/{len(tiles)} tiles")

    # terrarium decode: meters = (R*256 + G + B/256) - 32768
    elev = canvas[:, :, 0] * 256.0 + canvas[:, :, 1] + canvas[:, :, 2] / 256.0 - 32768.0

    # crop the stitched canvas to the exact bbox (sub-tile precision)
    px_w, px_h = nx * TILE, ny * TILE
    fx0, fx1 = (lon2x(WEST, ZOOM) - x0) * TILE, (lon2x(EAST, ZOOM) - x0) * TILE
    fy0, fy1 = (lat2y(NORTH, ZOOM) - y0) * TILE, (lat2y(SOUTH, ZOOM) - y0) * TILE
    cl = lambda v, hi: max(0, min(hi, int(round(v))))
    elev = elev[cl(fy0, px_h):cl(fy1, px_h), cl(fx0, px_w):cl(fx1, px_w)]

    land = elev > 0.5
    elev_land = np.where(land, elev, 0.0)
    max_m = float(elev_land.max()) if land.any() else 1.0
    print(f"cropped {elev.shape[1]}x{elev.shape[0]} px | land max {max_m:.0f} m | "
          f"land {100*land.mean():.1f}%")

    # --- 16-bit heightmap, split across R(hi)/G(lo) for lossless load in Godot ---
    h16 = np.zeros(elev.shape, dtype=np.uint16)
    h16[land] = np.clip(elev_land[land] / max_m * 65535.0, 1, 65535).astype(np.uint16)
    rg = np.zeros((*h16.shape, 3), dtype=np.uint8)
    rg[:, :, 0] = (h16 >> 8) & 0xFF
    rg[:, :, 1] = h16 & 0xFF
    Image.fromarray(rg, "RGB").save(OUT / "heightmap_rg.png")

    # --- hillshade (sun from NW) ---
    mpp = (EAST - WEST) * 111320 * math.cos(math.radians((NORTH + SOUTH) / 2)) / elev.shape[1]
    gy, gx = np.gradient(elev_land, mpp)
    slope = np.pi / 2.0 - np.arctan(np.hypot(gx, gy))
    aspect = np.arctan2(-gx, gy)
    az, alt = math.radians(315), math.radians(45)
    shade = np.clip(np.sin(alt) * np.sin(slope) +
                    np.cos(alt) * np.cos(slope) * np.cos(az - aspect), 0, 1)

    # --- painted relief texture: elevation colour * hillshade, blue sea ---
    t = np.sqrt(np.clip(elev_land / max_m, 0, 1))
    r, g, b = colour_ramp(t)
    lit = 0.55 + 0.6 * shade                                  # relief brightness
    rgb_relief = np.zeros((*elev.shape, 3), dtype=np.float64)
    rgb_relief[..., 0] = np.where(land, np.clip(r * lit, 0, 1), 0.13)
    rgb_relief[..., 1] = np.where(land, np.clip(g * lit, 0, 1), 0.28)
    rgb_relief[..., 2] = np.where(land, np.clip(b * lit, 0, 1), 0.45)
    Image.fromarray((rgb_relief * 255).astype(np.uint8), "RGB").save(OUT / "relief.png")

    # small preview
    prev = Image.fromarray((rgb_relief * 255).astype(np.uint8), "RGB")
    prev.thumbnail((900, 1600))
    prev.save(OUT / "hillshade.png")

    json.dump({
        "bbox": [WEST, SOUTH, EAST, NORTH], "zoom": ZOOM,
        "width": elev.shape[1], "height": elev.shape[0],
        "max_elevation_m": max_m,
        "approx_m_per_px_x": mpp,
        "approx_m_per_px_y": (NORTH - SOUTH) * 110540 / elev.shape[0],
        "land_fraction": float(land.mean()),
    }, open(OUT / "meta.json", "w"), indent=2)
    print("wrote heightmap_rg.png, relief.png, hillshade.png, meta.json")


if __name__ == "__main__":
    main()
