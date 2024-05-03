mod npgeohash;
use npgeohash::Bounds;
use numpy::ndarray::*;
use numpy::*;
use pyo3::prelude::*;

fn to_numpy_string_array(py: Python, vec: Vec<String>) -> Bound<'_, PyArray1<PyFixedUnicode<12>>> {
    let v = vec
        .into_iter()
        .map(|s| {
            let mut ch = [0u32; 12];
            let unicodes: Vec<u32> = s.chars().take(12).map(u32::from).collect();
            ch[..unicodes.len()].copy_from_slice(&unicodes);
            ch.into()
        })
        .collect();
    PyArray1::from_vec_bound(py, v)
}

#[pyfunction]
fn encode(lat: f64, lon: f64, precision: u32) -> String {
    npgeohash::encode(lat, lon, precision)
}

#[pyfunction]
fn encode_array<'py>(py: Python<'py>, arr: PyReadonlyArrayDyn<'py, f64>, precision: u32) -> Vec<String> {
    let arr = arr.as_array();
    let mut ret = Vec::new();
    for a in arr.axis_iter(Axis(0)) {
        let lat = a.get(0).unwrap();
        let lon = a.get(1).unwrap();
        ret.push(npgeohash::encode(*lat, *lon, precision));
    }
    ret
}

#[pyfunction]
fn to_bounds(code: &str) -> (f64, f64, f64, f64) {
    let bounds = npgeohash::to_bounds(code);
    (bounds.lat_min, bounds.lon_min, bounds.lat_max, bounds.lon_max)
}

#[pyfunction]
fn neighbors(code: &str) -> Vec<String> {
    npgeohash::neighbors(code)
}

#[pyfunction]
fn create_rect(bounds: (f64, f64, f64, f64), precision: u32) -> Vec<String> {
    npgeohash::create_rect(
        &Bounds {
            lat_min: bounds.0,
            lon_min: bounds.1,
            lat_max: bounds.2,
            lon_max: bounds.3,
        },
        precision,
    )
}

#[pyfunction]
fn create_circle(lat: f64, lon: f64, radius: f64, precision: u32) -> Vec<String> {
    npgeohash::create_circle(lat, lon, radius, precision)
}

#[pyfunction]
fn compress(codes: Vec<String>, accuracy: f64) -> Vec<String> {
    let mut v = codes.iter().map(|c| c.as_str()).collect();
    npgeohash::compress(&mut v, accuracy);
    v.into_iter().map(|c| c.to_string()).collect()
}

#[pyfunction]
fn many_neighbors(codes: Vec<String>) -> Vec<String> {
    codes.into_iter().flat_map(|code| npgeohash::neighbors(&code)).collect()
}

#[pyfunction]
fn isin(poi: Vec<String>, codes: Vec<String>) -> Vec<bool> {
    poi.iter().map(|x| codes.iter().any(|y| x.starts_with(y) || y.starts_with(x))).collect()
}

#[pyfunction]
fn isin_circle(poi: Vec<String>, lat: f64, lon: f64, radius: f64, precision: u32) -> Vec<bool> {
    isin(poi, npgeohash::create_circle(lat, lon, radius, precision))
}

#[pymodule]
fn _npgeohash_rs(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(encode, m)?)?;
    m.add_function(wrap_pyfunction!(encode_array, m)?)?;
    m.add_function(wrap_pyfunction!(to_bounds, m)?)?;
    m.add_function(wrap_pyfunction!(neighbors, m)?)?;
    m.add_function(wrap_pyfunction!(create_rect, m)?)?;
    m.add_function(wrap_pyfunction!(create_circle, m)?)?;
    m.add_function(wrap_pyfunction!(compress, m)?)?;
    m.add_function(wrap_pyfunction!(many_neighbors, m)?)?;
    m.add_function(wrap_pyfunction!(isin, m)?)?;
    m.add_function(wrap_pyfunction!(isin_circle, m)?)?;
    Ok(())
}
