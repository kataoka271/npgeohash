from collections import defaultdict
from math import ceil, cos, floor, pi, sqrt
from typing import Iterable, Iterator

import numpy as np
from numba import njit
from numpy.typing import NDArray

base = "0123456789bcdefghjkmnpqrstuvwxyz"


@njit
def create_circle(lat, lon, radius, precision) -> Iterator[str]:
    code = encode(lat, lon, precision)
    lat_max, lat_min, lon_max, lon_min = to_latlon(code)
    w, h = to_distance(lat_max - lat_min, lon_max - lon_min, lat)
    lat_bits, lon_bits = split_latlon(code)
    rx, ry = (lon - lon_min) / (lon_max - lon_min), (lat - lat_min) / (lat_max - lat_min)
    pts = gridpoints(rx, ry, radius, w, h)
    for i, j in pts:
        yield join_latlon(lat_bits + j, lon_bits + i, precision)


@njit
def gridpoints(a, b, r, w, h) -> Iterator[tuple[int, int]]:
    def f(x):
        return pow(r, 2) - pow((a - x) * w, 2)

    x = ceil(a - r / w) - 1
    x_last = floor(a + r / w)
    while x <= x_last:
        p = sqrt(max(f(x + 1), f(x))) / h
        y = ceil(b - p) - 1
        y_last = floor(b + p)
        while y <= y_last:
            yield (x, y)
            y += 1
        x += 1


@njit
def to_distance(lat_diff, lon_diff, lat) -> tuple[float, float]:
    R = 6378137  # 赤道半径
    w = lon_diff * (pi / 180.0) * R * cos(lat * pi / 180.0)
    h = lat_diff * (pi / 180.0) * R
    return (w, h)


@njit
def to_latlon(code) -> tuple[float, float, float, float]:
    lat_max = 90
    lat_min = -90
    lon_max = 180
    lon_min = -180
    v = 0
    for c in code:
        v = (v << 5) | base.index(c)
    i = len(code) * 5 - 1
    while i >= 0:
        lon_mid = (lon_max + lon_min) / 2
        if (v >> i) & 1 == 1:
            lon_min = lon_mid
        else:
            lon_max = lon_mid
        i = i - 1
        if i < 0:
            break
        lat_mid = (lat_max + lat_min) / 2
        if (v >> i) & 1 == 1:
            lat_min = lat_mid
        else:
            lat_max = lat_mid
        i = i - 1
    return (lat_max, lat_min, lon_max, lon_min)


@njit
def neighbors(code) -> list[str]:
    precision = len(code)
    lat, lon = split_latlon(code)
    north = join_latlon(lat + 1, lon, precision)
    west = join_latlon(lat, lon - 1, precision)
    south = join_latlon(lat - 1, lon, precision)
    east = join_latlon(lat, lon + 1, precision)
    nw = join_latlon(lat + 1, lon - 1, precision)
    sw = join_latlon(lat - 1, lon - 1, precision)
    se = join_latlon(lat - 1, lon + 1, precision)
    ne = join_latlon(lat + 1, lon + 1, precision)
    return [code, north, nw, west, sw, south, se, east, ne]


@njit
def split_latlon(code) -> tuple[float, float]:
    lat = 0
    lon = 0
    v = 0
    for c in code:
        v = (v << 5) | base.index(c)
    i = len(code) * 5 - 1
    while i >= 0:
        lon = (lon << 1) | ((v >> i) & 1)
        i = i - 1
        if i < 0:
            break
        lat = (lat << 1) | ((v >> i) & 1)
        i = i - 1
    return (lat, lon)


@njit
def join_latlon(lat, lon, precision) -> str:
    nbits = precision * 5
    c = []
    i = nbits // 2 - 1
    v = 0
    if nbits % 2 == 1:
        v = (v << 1) | ((lon >> i) & 1)
        while i >= 0:
            v = (v << 1) | ((lat >> i) & 1)
            v = (v << 1) | ((lon >> i) & 1)
            i -= 1
    else:
        while i >= 0:
            v = (v << 1) | ((lon >> i) & 1)
            v = (v << 1) | ((lat >> i) & 1)
            i -= 1
    for i in range(nbits - 5, -5, -5):
        c.append(base[(v >> i) & 0x1F])
    return "".join(c)


@njit
def encode(lat, lon, precision) -> str:
    lat_max = 90
    lat_min = -90
    lon_max = 180
    lon_min = -180
    value = 0
    i = 0
    code = []
    nbits = precision * 5

    # latitude, longitude
    while i < nbits:
        lon_mid = (lon_max + lon_min) / 2
        if lon_mid <= lon:
            value = (value << 1) | 1
            lon_min = lon_mid
        else:
            value = value << 1
            lon_max = lon_mid
        i += 1
        if i % 5 == 0:
            code.append(base[value & 0x1F])
        if i >= nbits:
            break
        lat_mid = (lat_max + lat_min) / 2
        if lat_mid <= lat:
            value = (value << 1) | 1
            lat_min = lat_mid
        else:
            value = value << 1
            lat_max = lat_mid
        i += 1
        if i % 5 == 0:
            code.append(base[value & 0x1F])

    return "".join(code)


dtype = "U12"  # np.dtype([("hash", "i8"), ("code", "U10")])


@njit
def encode_array(array: NDArray[np.float_], precision: int) -> NDArray[np.str_]:
    values = np.empty(array.shape[0], dtype=dtype)
    for i in range(array.shape[0]):
        values[i] = encode(array[i, 0], array[i, 1], precision)
    return values


@njit
def isin(poi: NDArray[np.str_], codes: NDArray[np.str_]) -> NDArray[np.bool8]:
    arr = np.full(poi.shape[0], False)
    for i in range(poi.shape[0]):
        for j in range(codes.shape[0]):
            s = poi[i]
            t = codes[j]
            arr[i] = arr[i] or s.startswith(t) or t.startswith(s)
    return arr


# the following functions cannot support numba compilation due to unsupported type/operation needed


def isin_circle(poi: NDArray[np.str_], lat: float, lon: float, radius: float, precision: int) -> NDArray[np.bool8]:
    return isin(poi, np.fromiter(create_circle(lat, lon, radius, precision), dtype=dtype))


def many_neighbors(codes: Iterable[str]) -> set[str]:
    S = set()
    for code in codes:
        S.update(neighbors(code))
    return S


def compress(codes: Iterable[str], *, accuracy: float = 1.0) -> list[str]:
    input_codes = list(codes)
    while True:
        d = defaultdict(list)
        for c in input_codes:
            d[c[:-1]].append(c)
        compressed = []
        for c, v in d.items():
            if len(v) >= 32 * accuracy:
                compressed.append(c)
            else:
                compressed.extend(v)
        if len(input_codes) == len(compressed):
            break
        input_codes = compressed
    return compressed
