# npgeohash - Create Geohash on NumPy

## Installation

```shell
poetry add git+https://github.com/kataoka271/npgeohash.git
```
or
```shell
pip install git+https://github.com/kataoka271/npgeohash.git
```

## Usage


```python
import os
import time
from typing import Iterable

import numpy as np
from folium import Circle, Icon, Map, Marker, Rectangle
from IPython.core.display import Markdown
from selenium import webdriver

import npgeohash.npgeohash as npgeohash


def drawbox(m: Map, codes: Iterable[str], color: str) -> Map:
    for code in codes:
        lat_min, lon_min, lat_max, lon_max = npgeohash.to_latlon(code)
        center = ((lat_max, lon_max), (lat_min, lon_min))
        Rectangle(center, fill=True, fill_opacity=0.3, fill_color=color, color=color).add_to(m)
    return m


CACHE = 1

if CACHE == 1:
    image_index = 0

    def showmap(m: Map):
        global image_index
        filename = f"images/{image_index:02d}.png"
        display(Markdown(f"![]({filename})"))
        image_index += 1

elif CACHE == 2:
    image_index = 0

    def showmap(m: Map):
        global image_index
        filename = f"images/{image_index:02d}.png"
        htmlfile = "map.html"
        m.save(htmlfile)
        browser = webdriver.Firefox()
        browser.get(f"file://{os.getcwd()}/{htmlfile}")
        time.sleep(5)
        browser.save_screenshot(filename)
        browser.quit()
        os.unlink(htmlfile)
        display(Markdown(f"![]({filename})"))
        image_index += 1

else:

    def showmap(m: Map):
        display(m)


arr = np.array(
    [
        [35.689655888210886, 139.70010995782644],
        [35.72985642217818, 139.71048000669535],
        [35.68563034485054, 139.76282021384134],
    ]
)
arr
```




    array([[ 35.68965589, 139.70010996],
           [ 35.72985642, 139.71048001],
           [ 35.68563034, 139.76282021]])



## npgeohash.encode_array(array, precision)


```python
geohashes = npgeohash.encode_array(arr, 7)
geohashes
```




    array(['xn774cn', 'xn7770q', 'xn77h2k'], dtype='<U12')




```python
m = Map(zoom_start=15)
drawbox(m, geohashes, "red")
m.fit_bounds(m.get_bounds())
for latlon in arr:
    Marker(latlon).add_to(m)
showmap(m)
```


![](images/00.png)


## npgeohash.neighbors(code)


```python
nei = npgeohash.neighbors(geohashes[0])
print(nei)
```

    ['xn774cn', 'xn774cq', 'xn774cm', 'xn774cj', 'xn774bv', 'xn774by', 'xn774bz', 'xn774cp', 'xn774cr']
    


```python
m = Map(zoom_start=15)
drawbox(m, nei, "blue")
drawbox(m, [geohashes[0]], "red")
m.fit_bounds(m.get_bounds())
showmap(m)
```


![](images/01.png)


## npgeohash.create_circle(latitude, longitude, radius, precision)

Note that `create_circle` is generator iterating circular geohashes.


```python
lat, lon = arr[0]
cir = list(npgeohash.create_circle(lat, lon, 1000, 7))
print(cir)
```

    ['xn7748w', 'xn7748y', 'xn7749n', 'xn7749q', 'xn7749w', 'xn7748p', 'xn7748r', 'xn7748x', 'xn7748z', 'xn7749p', 'xn7749r', 'xn7749x', 'xn7749z', 'xn774dp', 'xn76fzb', 'xn774b0', 'xn774b2', 'xn774b8', 'xn774bb', 'xn774c0', 'xn774c2', 'xn774c8', 'xn774cb', 'xn774f0', 'xn774f2', 'xn76fz9', 'xn76fzc', 'xn774b1', 'xn774b3', 'xn774b9', 'xn774bc', 'xn774c1', 'xn774c3', 'xn774c9', 'xn774cc', 'xn774f1', 'xn774f3', 'xn76fzd', 'xn76fzf', 'xn774b4', 'xn774b6', 'xn774bd', 'xn774bf', 'xn774c4', 'xn774c6', 'xn774cd', 'xn774cf', 'xn774f4', 'xn774f6', 'xn774fd', 'xn76fze', 'xn76fzg', 'xn774b5', 'xn774b7', 'xn774be', 'xn774bg', 'xn774c5', 'xn774c7', 'xn774ce', 'xn774cg', 'xn774f5', 'xn774f7', 'xn774fe', 'xn76fzk', 'xn76fzs', 'xn76fzu', 'xn774bh', 'xn774bk', 'xn774bs', 'xn774bu', 'xn774ch', 'xn774ck', 'xn774cs', 'xn774cu', 'xn774fh', 'xn774fk', 'xn774fs', 'xn76fzm', 'xn76fzt', 'xn76fzv', 'xn774bj', 'xn774bm', 'xn774bt', 'xn774bv', 'xn774cj', 'xn774cm', 'xn774ct', 'xn774cv', 'xn774fj', 'xn774fm', 'xn774ft', 'xn76fzq', 'xn76fzw', 'xn76fzy', 'xn774bn', 'xn774bq', 'xn774bw', 'xn774by', 'xn774cn', 'xn774cq', 'xn774cw', 'xn774cy', 'xn774fn', 'xn774fq', 'xn774fw', 'xn76fzr', 'xn76fzx', 'xn76fzz', 'xn774bp', 'xn774br', 'xn774bx', 'xn774bz', 'xn774cp', 'xn774cr', 'xn774cx', 'xn774cz', 'xn774fp', 'xn774fr', 'xn774fx', 'xn76gp2', 'xn76gp8', 'xn76gpb', 'xn77500', 'xn77502', 'xn77508', 'xn7750b', 'xn77510', 'xn77512', 'xn77518', 'xn7751b', 'xn77540', 'xn77542', 'xn77548', 'xn76gp9', 'xn76gpc', 'xn77501', 'xn77503', 'xn77509', 'xn7750c', 'xn77511', 'xn77513', 'xn77519', 'xn7751c', 'xn77541', 'xn77543', 'xn77549', 'xn76gpd', 'xn76gpf', 'xn77504', 'xn77506', 'xn7750d', 'xn7750f', 'xn77514', 'xn77516', 'xn7751d', 'xn7751f', 'xn77544', 'xn77546', 'xn7754d', 'xn76gpe', 'xn76gpg', 'xn77505', 'xn77507', 'xn7750e', 'xn7750g', 'xn77515', 'xn77517', 'xn7751e', 'xn7751g', 'xn77545', 'xn77547', 'xn76gpu', 'xn7750h', 'xn7750k', 'xn7750s', 'xn7750u', 'xn7751h', 'xn7751k', 'xn7751s', 'xn7751u', 'xn7754h', 'xn7754k', 'xn7750j', 'xn7750m', 'xn7750t', 'xn7750v', 'xn7751j', 'xn7751m', 'xn7751t', 'xn7751v', 'xn7754j', 'xn7750q', 'xn7750w', 'xn7750y', 'xn7751n', 'xn7751q', 'xn7751w']
    


```python
m = Map(zoom_start=15)
drawbox(m, cir, "blue")
Marker(arr[0]).add_to(m)
Circle(arr[0], 1000, color="yellow").add_to(m)
m.fit_bounds(m.get_bounds())
showmap(m)
```


![](images/02.png)


## npgeohash.isin_circle(poi, latitude, longitude, radius, precision)

This function checks geohashes is within radius on (`latitude`, `longitude`).


```python
poi = np.array(
    [
        [35.69240645093, 139.7034750767164],
        [35.691255831981294, 139.69025228754268],
        [35.68307317410146, 139.71622562341963],
        [35.70071598380815, 139.69749333875686],
        [35.68997734701496, 139.6847427920536],
        [35.68115524225217, 139.68584469115146],
    ]
)
lat, lon = 35.68952987243547, 139.69953972279566

poi_geohashes = npgeohash.encode_array(poi, 7)
poi_geohashes
```




    array(['xn77518', 'xn7749r', 'xn76grf', 'xn774gh', 'xn77495', 'xn76fxs'],
          dtype='<U12')




```python
isin = npgeohash.isin_circle(poi_geohashes, lat, lon, 1000, 7)
isin
```




    array([ True,  True, False, False, False, False])




```python
cir = npgeohash.create_circle(lat, lon, 1000, 7)

m = Map(zoom_start=15)

drawbox(m, cir, "blue")

Marker([lat, lon], icon=Icon(icon="home", color="blue")).add_to(m)
Circle([lat, lon], 1000, color="yellow").add_to(m)

for contain, latlon in zip(isin, poi):
    if contain:
        Marker(latlon, icon=Icon(icon="ok", color="green")).add_to(m)
    else:
        Marker(latlon, icon=Icon(icon="remove", color="red")).add_to(m)

m.fit_bounds(m.get_bounds())
showmap(m)
```


![](images/03.png)


## npgeohash.many_neighbors(codes)


```python
nei = npgeohash.many_neighbors(npgeohash.many_neighbors(geohashes))
print(nei)
```

    {'xn77h2w', 'xn775rb', 'xn775pu', 'xn77h2t', 'xn77h2s', 'xn7770h', 'xn775pv', 'xn77h2h', 'xn774ck', 'xn7770p', 'xn77518', 'xn774bu', 'xn77510', 'xn7770x', 'xn76ury', 'xn774bv', 'xn77h2u', 'xn76urf', 'xn77722', 'xn7770y', 'xn77728', 'xn77h2v', 'xn7770k', 'xn774cj', 'xn76uru', 'xn77h2k', 'xn77h24', 'xn774cr', 'xn77h2m', 'xn7770z', 'xn7770r', 'xn7772b', 'xn77h2n', 'xn7770j', 'xn7770v', 'xn7770s', 'xn774cs', 'xn77h2f', 'xn7770t', 'xn775pz', 'xn76urv', 'xn77h2q', 'xn77720', 'xn774cx', 'xn774cm', 'xn7770w', 'xn774bs', 'xn77h25', 'xn774bt', 'xn7770m', 'xn774bz', 'xn76urg', 'xn7770q', 'xn774cp', 'xn774ch', 'xn774bx', 'xn77h2d', 'xn77h2j', 'xn77h2e', 'xn77h27', 'xn7750b', 'xn774cw', 'xn774bw', 'xn774cq', 'xn77h26', 'xn77h2y', 'xn7770u', 'xn774ct', 'xn774cn', 'xn77512', 'xn775py', 'xn7770n', 'xn774by', 'xn77h2g', 'xn77508'}
    


```python
m = Map(zoom_start=15)
drawbox(m, nei, "blue")
m.fit_bounds(m.get_bounds())
showmap(m)
```


![](images/04.png)


## npgeohash.compress(codes, accuracy=1.0)

The following example is accurate geohash compression.


```python
lat, lon = arr[0]

cir = npgeohash.create_circle(lat, lon, 1000, 7)
compressed = npgeohash.compress(cir, accuracy=1.0)

m = Map()
drawbox(m, compressed, "blue")
Marker(arr[0]).add_to(m)
Circle(arr[0], 1000, color="yellow").add_to(m)
m.fit_bounds(m.get_bounds())
showmap(m)
```


![](images/05.png)


The following example is obscured geohash compression.


```python
cir = npgeohash.create_circle(lat, lon, 1000, 7)
compressed = npgeohash.compress(cir, accuracy=0.6)

m = Map()
drawbox(m, compressed, "blue")
Marker(arr[0]).add_to(m)
Circle(arr[0], 1000, color="yellow").add_to(m)
m.fit_bounds(m.get_bounds())
showmap(m)
```


![](images/06.png)


The compression is executed recursively.


```python
cir = npgeohash.create_circle(lat, lon, 1000, 8)
compressed = npgeohash.compress(cir, accuracy=1.0)

m = Map()
drawbox(m, compressed, "blue")
Marker([lat, lon]).add_to(m)
Circle([lat, lon], 1000, color="yellow").add_to(m)
m.fit_bounds(m.get_bounds())
showmap(m)
```


![](images/07.png)


Obsured recursive compress is executed as below.


```python
cir = npgeohash.create_circle(lat, lon, 1000, 8)
compressed = npgeohash.compress(cir, accuracy=0.6)

m = Map()
drawbox(m, compressed, "blue")
Marker([lat, lon]).add_to(m)
Circle([lat, lon], 1000, color="yellow").add_to(m)
m.fit_bounds(m.get_bounds())
showmap(m)
```


![](images/08.png)


## npgeohash.isin(poi, codes)


```python
cir = npgeohash.create_circle(lat, lon, 1000, 8)
compressed = npgeohash.compress(cir, accuracy=1.0)
isin = npgeohash.isin(poi_geohashes, compressed)

m = Map()
drawbox(m, compressed, "blue")
Marker([lat, lon]).add_to(m)
Circle([lat, lon], 1000, color="yellow").add_to(m)
for contain, latlon in zip(isin, poi):
    if contain:
        Marker(latlon, icon=Icon(icon="ok", color="green")).add_to(m)
    else:
        Marker(latlon, icon=Icon(icon="remove", color="red")).add_to(m)
m.fit_bounds(m.get_bounds())
showmap(m)
```


![](images/09.png)


If you need more performance, Numba jitted functions are available in `npgeohash_jit`.
However, keep your in mind it takes several seconds to complete compiling functions.

See [performance.ipynb](./docs/performance.ipynb) for performance comparison.


```python
from npgeohash import npgeohash_jit as npgeohash

geohashes = npgeohash.encode_array(arr, 7)
geohashes
```




    array(['xn774cn', 'xn7770q', 'xn77h2k'], dtype='<U12')



## npgeohash.create_box(box, precision)


```python
box = (51.648364, -0.410270, 51.944039, 0.277987)

m = Map()
drawbox(m, npgeohash.create_box(box, 6), "blue")
m.add_child(Marker(location=(box[0], box[1])))
m.add_child(Marker(location=(box[2], box[3])))
m.fit_bounds(m.get_bounds())
showmap(m)
```


![](images/10.png)

