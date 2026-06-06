#!/usr/bin/env python3
"""Rasterize PH region boundaries into a province-ID map aligned to the heightmap.

Source: geoBoundaries PHL ADM1 (gbOpen, CC BY 4.0 — commercial-safe). The 17
regions map 1:1 onto the engine's region ids (see data.rs). Output:
  - data/provinces.png        : R channel = engine region id (0 = sea/unassigned)
  - data/provinces_debug.png  : distinct colour per region (alignment eyeball check)
  - data/adm1.geojson         : cached boundary download

Alignment: the heightmap pixels are Web-Mercator (from terrain tiles) cropped to
the bbox, so we project polygon lon/lat with the SAME mapping — Mercator x is
linear in lon; y uses asinh(tan(lat)). Ratios cancel the zoom factor.
"""
import json, math, urllib.request, colorsys
from pathlib import Path
from PIL import Image, ImageDraw

OUT = Path(__file__).parent / "data"
GJ_URL = ("https://github.com/wmgeolab/geoBoundaries/raw/41af8f1/"
          "releaseData/gbOpen/PHL/ADM1/geoBoundaries-PHL-ADM1.geojson")

# geoBoundaries shapeName -> engine region id (data.rs philippines_regions order)
NAME_TO_ID = {
    "NCR": 1, "CAR": 2, "Ilocos Region": 3, "Cagayan Valley": 4,
    "Central Luzon": 5, "Calabarzon": 6, "Mimaropa": 7, "Bicol Region": 8,
    "Western Visayas": 9, "Central Visayas": 10, "Eastern Visayas": 11,
    "Zamboanga Peninsula": 12, "Northern Mindanao": 13, "Davao Region": 14,
    "Soccsksargen": 15, "Caraga": 16, "ARMM": 17,
}


def merc_y(lat):
    return math.asinh(math.tan(math.radians(lat)))


def main():
    meta = json.load(open(OUT / "meta.json"))
    W, H = meta["width"], meta["height"]
    west, south, east, north = meta["bbox"]
    yN, yS = merc_y(north), merc_y(south)

    cache = OUT / "adm1.geojson"
    if not cache.exists():
        data = urllib.request.urlopen(
            urllib.request.Request(GJ_URL, headers={"User-Agent": "politiko-spike"}),
            timeout=120).read()
        cache.write_bytes(data)
    gj = json.loads(cache.read_text())

    def to_px(lon, lat):
        col = (lon - west) / (east - west) * W
        row = (yN - merc_y(lat)) / (yN - yS) * H
        return (col, row)

    id_img = Image.new("RGB", (W, H), (0, 0, 0))
    dbg_img = Image.new("RGB", (W, H), (18, 40, 66))
    id_draw = ImageDraw.Draw(id_img)
    dbg_draw = ImageDraw.Draw(dbg_img)

    palette = {}
    for i in range(1, 18):
        r, g, b = colorsys.hsv_to_rgb((i * 0.61803) % 1.0, 0.55, 0.85)
        palette[i] = (int(r * 255), int(g * 255), int(b * 255))

    matched, missing = [], []
    for feat in gj["features"]:
        name = feat["properties"].get("shapeName", "?")
        rid = NAME_TO_ID.get(name)
        if rid is None:
            missing.append(name)
            continue
        matched.append((name, rid))
        geom = feat["geometry"]
        polys = (geom["coordinates"] if geom["type"] == "MultiPolygon"
                 else [geom["coordinates"]])
        for poly in polys:
            ring = [to_px(lon, lat) for lon, lat in poly[0]]  # exterior ring
            if len(ring) >= 3:
                id_draw.polygon(ring, fill=(rid, 0, 0))
                dbg_draw.polygon(ring, fill=palette[rid])

    id_img.save(OUT / "provinces.png")
    dbg_img.save(OUT / "provinces_debug.png")
    print(f"rasterized {len(matched)}/17 regions at {W}x{H}")
    if missing:
        print("UNMATCHED:", missing)
    print("wrote provinces.png, provinces_debug.png")


if __name__ == "__main__":
    main()
