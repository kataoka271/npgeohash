from typing import TypeVar

from numba import njit
from numba.extending import register_jitable

from npgeohash import npgeohash

T = TypeVar("T")


def _jit(sig: str, f: T) -> T:
    return njit(sig, cache=True)(f)  # type: ignore


def _register(sig: str, f: T) -> T:
    return _jit(sig, register_jitable(f))  # type: ignore


encode = _register("(f8, f8, i8)", npgeohash.encode)

to_bounds = _register("Tuple((f8, f8, f8, f8))(string)", npgeohash.to_bounds)

_gridsize = _register("(f8, f8, f8)", npgeohash._gridsize)

_split_bits = _register("Tuple((i8, i8))(string)", npgeohash._split_bits)

_join_bits = _register("string(i8, i8, i8)", npgeohash._join_bits)

_gridpoints = _register("(f8, f8, f8, f8, f8)", npgeohash._gridpoints)

create_rect = _jit("(Tuple((f8, f8, f8, f8)), i8)", npgeohash.create_rect)

create_circle = _jit("(f8, f8, f8, i8)", npgeohash.create_circle)

neighbors = _jit("List(string)(string)", npgeohash.neighbors)

encode_array = _jit("(f8[:, :], i8)", npgeohash.encode_array)

_isin = _jit("(string[:], string[:])", npgeohash._isin)

isin = npgeohash.isin

isin_circle = npgeohash.isin_circle

many_neighbors = npgeohash.many_neighbors

compress = npgeohash.compress
