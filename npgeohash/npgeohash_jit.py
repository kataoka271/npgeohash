from typing import TypeVar

from numba import jit
from numba.extending import register_jitable

from npgeohash import npgeohash

T = TypeVar("T")


def _jit(sig: str, f: T) -> T:
    return jit(sig, nopython=True)(f)  # type: ignore


def _register(sig: str, f: T) -> T:
    return _jit(sig, register_jitable(f))  # type: ignore


encode = _register("(f8,f8,i8)", npgeohash.encode)

to_latlon = _register("Tuple((f8,f8,f8,f8))(string)", npgeohash.to_latlon)

_to_distance = _register("(f8,f8,f8)", npgeohash._to_distance)

_split_latlon_bin = _register("Tuple((i8,i8))(string)", npgeohash._split_latlon_bin)

_join_latlon_bin = _register("string(i8,i8,i8)", npgeohash._join_latlon_bin)

_gridpoints = _register("(f8,f8,f8,f8,f8)", npgeohash._gridpoints)

create_box = _jit("(Tuple((f8,f8,f8,f8)),i8)", npgeohash.create_box)

create_circle = _jit("(f8,f8,f8,i8)", npgeohash.create_circle)

neighbors = _jit("List(string)(string)", npgeohash.neighbors)

encode_array = _jit("(f8[:,:],i8)", npgeohash.encode_array)

_isin = _jit("(string[:],string[:])", npgeohash._isin)

isin = npgeohash.isin

isin_circle = npgeohash.isin_circle

many_neighbors = npgeohash.many_neighbors

compress = npgeohash.compress
