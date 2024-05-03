use std::{collections::HashMap, f64::consts::PI};

const BASE: &[u8] = b"0123456789bcdefghjkmnpqrstuvwxyz";

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Bounds {
    pub lat_min: f64,
    pub lon_min: f64,
    pub lat_max: f64,
    pub lon_max: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct LatLonBits {
    lat: u64,
    lon: u64,
}

impl LatLonBits {
    fn add(&self, x: i64, y: i64) -> Self {
        LatLonBits {
            lat: (self.lat as i64 + y) as u64,
            lon: (self.lon as i64 + x) as u64,
        }
    }
}

pub fn encode(lat: f64, lon: f64, precision: u32) -> String {
    let mut lat_max = 90.0;
    let mut lat_min = -90.0;
    let mut lon_max = 180.0;
    let mut lon_min = -180.0;
    let mut value = 0;
    let mut i = 0;
    let mut code = Vec::new();
    let nbits = precision * 5;

    // latitude, longitude
    while i < nbits {
        let lon_mid = (lon_max + lon_min) / 2.0;
        if lon_mid <= lon {
            value = (value << 1) | 1;
            lon_min = lon_mid;
        } else {
            value <<= 1;
            lon_max = lon_mid;
        }
        if i % 5 == 4 {
            code.push(BASE[value & 0x1F]);
        }
        i += 1;
        if i >= nbits {
            break;
        }
        let lat_mid = (lat_max + lat_min) / 2.0;
        if lat_mid <= lat {
            value = (value << 1) | 1;
            lat_min = lat_mid;
        } else {
            value <<= 1;
            lat_max = lat_mid;
        }
        if i % 5 == 4 {
            code.push(BASE[value & 0x1F]);
        }
        i += 1;
    }

    String::from_utf8(code).unwrap()
}

fn to_value(code: &str) -> u64 {
    code.bytes().fold(0, |v, c| (v << 5) | BASE.iter().position(|x| *x == c).unwrap() as u64)
}

fn from_value(value: u64, precision: u32) -> String {
    String::from_utf8((0..precision).map(|i| BASE[((value >> (5 * i)) & 0x1F) as usize]).rev().collect()).unwrap()
}

pub fn to_bounds(code: &str) -> Bounds {
    let mut lat_max = 90.0;
    let mut lat_min = -90.0;
    let mut lon_max = 180.0;
    let mut lon_min = -180.0;

    let value = to_value(code);
    let mut i = code.len() as u32 * 5;

    while i >= 2 {
        i -= 1;
        let lon_mid = (lon_max + lon_min) / 2.0;
        if (value >> i) & 1 == 1 {
            lon_min = lon_mid;
        } else {
            lon_max = lon_mid;
        }
        i -= 1;
        let lat_mid = (lat_max + lat_min) / 2.0;
        if (value >> i) & 1 == 1 {
            lat_min = lat_mid;
        } else {
            lat_max = lat_mid;
        }
    }
    if i == 1 {
        i -= 1;
        let lon_mid = (lon_max + lon_min) / 2.0;
        if (value >> i) & 1 == 1 {
            lon_min = lon_mid;
        } else {
            lon_max = lon_mid;
        }
    }

    Bounds {
        lat_min,
        lon_min,
        lat_max,
        lon_max,
    }
}

fn split_bits(code: &str) -> LatLonBits {
    let mut lat: u64 = 0;
    let mut lon: u64 = 0;
    let value = to_value(code);

    let mut i = code.len() as u32 * 5;

    while i >= 2 {
        i -= 1;
        lon = (lon << 1) | ((value >> i) & 1);
        i -= 1;
        lat = (lat << 1) | ((value >> i) & 1);
    }
    if i == 1 {
        lon = (lon << 1) | (value & 1);
    }

    LatLonBits { lat, lon }
}

fn join_bits(bits: LatLonBits, precision: u32) -> String {
    let mut i_lat = precision * 5 / 2;
    let mut i_lon = i_lat + precision % 2;
    let mut value = 0;

    while i_lat != 0 {
        i_lon -= 1;
        value = (value << 1) | ((bits.lon >> i_lon) & 1);
        i_lat -= 1;
        value = (value << 1) | ((bits.lat >> i_lat) & 1);
    }
    if i_lon == 1 {
        value = (value << 1) | (bits.lon & 1);
    }

    from_value(value, precision)
}

#[test]
fn test_even_precision() {
    let (lat, lon) = (35.65803, 139.701636);
    let expect = String::from("xn76fgre");
    let precision = 8;

    let code = encode(lat, lon, precision);
    assert_eq!(code, expect);

    let bounds = to_bounds(&code);
    assert_eq!(
        bounds,
        Bounds {
            lat_min: 35.658016204833984,
            lon_min: 139.7014617919922,
            lat_max: 35.65818786621094,
            lon_max: 139.7018051147461
        }
    );

    assert_eq!(to_value(&code), 0b1110110100001110011001110011111011101101); // 40 bits
    assert_eq!(from_value(to_value(&code), precision), expect);
    assert_eq!(
        split_bits(&code),
        LatLonBits {
            lat: 0b10110010101101101011,
            lon: 0b11100011010101111110,
        }
    );
    assert_eq!(join_bits(split_bits(&code), precision), code);
}

#[test]
fn test_odd_precision() {
    let (lat, lon) = (35.65803, 139.701636);
    let expect = String::from("xn76fgr");
    let precision = 7;

    let code = encode(lat, lon, precision);
    assert_eq!(code, expect);

    let bounds = to_bounds(&code);
    assert_eq!(
        bounds,
        Bounds {
            lat_min: 35.657501220703125,
            lon_min: 139.70077514648438,
            lat_max: 35.65887451171875,
            lon_max: 139.7021484375
        }
    );

    assert_eq!(to_value(&code), 0b11101101000011100110011100111110111); // 35 bits
    assert_eq!(from_value(to_value(&code), precision), expect);
    assert_eq!(
        split_bits(&code),
        LatLonBits {
            lat: 0b10110010101101101,
            lon: 0b111000110101011111,
        }
    );
    assert_eq!(join_bits(split_bits(&code), precision), code);
}

pub fn neighbors(code: &str) -> Vec<String> {
    let precision = code.len() as u32;

    let bits = split_bits(code);
    let north = join_bits(bits.add(0, 1), precision);
    let nw = join_bits(bits.add(-1, 1), precision);
    let west = join_bits(bits.add(-1, 0), precision);
    let sw = join_bits(bits.add(-1, -1), precision);
    let south = join_bits(bits.add(0, -1), precision);
    let se = join_bits(bits.add(1, -1), precision);
    let east = join_bits(bits.add(1, 0), precision);
    let ne = join_bits(bits.add(1, 1), precision);

    vec![code.to_string(), north, nw, west, sw, south, se, east, ne]
}

pub fn create_rect(bounds: &Bounds, precision: u32) -> Vec<String> {
    let nw = split_bits(&encode(bounds.lat_min, bounds.lon_min, precision));
    let se = split_bits(&encode(bounds.lat_max, bounds.lon_max, precision));
    let mut codes = Vec::new();
    for lat in nw.lat..se.lat + 1 {
        for lon in nw.lon..se.lon + 1 {
            codes.push(join_bits(LatLonBits { lat, lon }, precision));
        }
    }
    codes
}

#[derive(Debug, PartialEq, PartialOrd)]
struct GridSize {
    width: f64,
    height: f64,
}

fn calc_gridsize(dx: f64, dy: f64, y: f64) -> GridSize {
    const R: f64 = 6378137.0; // 赤道半径
    GridSize {
        width: dx * PI / 180.0 * R * (y * PI / 180.0).cos(),
        height: dy * PI / 180.0 * R,
    }
}

fn calc_gridpoints(center_x: f64, center_y: f64, radius: f64, width: f64, height: f64) -> Vec<(i64, i64)> {
    let mut points = Vec::new();
    let x = (center_x - radius / width).ceil() as i64;
    let x_last = (center_x + radius / width).floor() as i64;
    for x in x - 1..x_last + 1 {
        let p1 = radius.powi(2) - ((center_x - x as f64) * width).powi(2);
        let p2 = radius.powi(2) - ((center_x - (x + 1) as f64) * width).powi(2);
        let p = p1.max(p2).sqrt() / height;
        let y = (center_y - p).ceil() as i64;
        let y_last = (center_y + p).floor() as i64;
        for y in y - 1..y_last + 1 {
            points.push((x, y))
        }
    }
    points
}

pub fn create_circle(lat: f64, lon: f64, radius: f64, precision: u32) -> Vec<String> {
    let code = encode(lat, lon, precision);
    let bounds = to_bounds(&code);
    let gridsize = calc_gridsize(bounds.lon_max - bounds.lon_min, bounds.lat_max - bounds.lat_min, lat);
    let bits = split_bits(&code);
    let gridpoints = calc_gridpoints(
        (lon - bounds.lon_min) / (bounds.lon_max - bounds.lon_min),
        (lat - bounds.lat_min) / (bounds.lat_max - bounds.lat_min),
        radius,
        gridsize.width,
        gridsize.height,
    );
    gridpoints.into_iter().map(|(i, j)| join_bits(bits.add(i, j), precision)).collect()
}

pub fn compress(codes: &mut Vec<&str>, accuracy: f64) {
    // println!("* compress {codes:?} {accuracy}");
    let mut is_compact = false;
    let mut prefixes: HashMap<&str, u32> = HashMap::new();
    while !is_compact {
        for code in codes.iter() {
            if code.len() >= 2 {
                prefixes.entry(&code[..code.len() - 1]).and_modify(|count| *count += 1).or_insert(1);
            }
        }
        // println!("codes: {:?}", codes);
        // println!("prefixes: {:?}", prefixes);
        // println!();
        let n = codes.len();
        let mut v: Vec<&str> = Vec::new();
        for code in codes.iter() {
            if codes.iter().any(|parent| code != parent && code.starts_with(parent)) {
                continue;
            }
            if code.len() >= 2 {
                let prefix = &code[..code.len() - 1];
                if (*prefixes.get(prefix).unwrap() as f64) < (BASE.len() as f64) * accuracy {
                    if !v.contains(code) {
                        v.push(code);
                    }
                } else if !v.contains(&prefix) {
                    v.push(prefix);
                }
            } else if !v.contains(code) {
                v.push(code);
            }
        }
        codes.clear();
        codes.extend(v);
        is_compact = n == codes.len();
        prefixes.clear();
    }
}

#[test]
fn test_compress() {
    let mut v = vec!["bcbcde", "bcbcde", "bcbcd", "bcbef"];
    compress(&mut v, 1.0);
    assert_eq!(v, vec!["bcbcd", "bcbef"]);

    let v = [
        vec![String::from("bb")],
        BASE.iter().map(|c| format!("bcbc{}", *c as char)).collect(),
        vec![String::from("be")],
    ]
    .concat();
    let mut v: Vec<&str> = v.iter().map(|s| s.as_str()).collect();
    compress(&mut v, 1.0);
    assert_eq!(v, vec!["bb", "bcbc", "be"]);

    let v = [
        vec![String::from("bb")],
        BASE[..26].iter().map(|c| format!("bcb{}", *c as char)).collect(),
        vec![String::from("be")],
    ]
    .concat();
    let mut v: Vec<&str> = v.iter().map(|s| s.as_str()).collect();
    compress(&mut v, 0.8);
    assert_eq!(v, vec!["bb", "bcb", "be"]);

    let v = [
        vec![String::from("bb")],
        BASE[..25]
            .iter()
            .flat_map(|c| BASE[..26].iter().map(|d| format!("bcb{}{}", *c as char, *d as char)))
            .collect(),
        vec![String::from("bcbt")],
    ]
    .concat();
    let mut v: Vec<&str> = v.iter().map(|s| s.as_str()).collect();
    compress(&mut v, 0.8);
    assert_eq!(v, vec!["bb", "bcb"]);

    let mut v = vec!["bc1", "bcb0", "bcb1", "bcc0", "bcc1"];
    compress(&mut v, 0.0625);
    assert_eq!(v, vec!["bc"]);

    let mut v = vec!["b1", "bb", "be", "bc", "bcb1", "bcb2", "bcb3", "bcb4"];
    compress(&mut v, 0.125);
    assert_eq!(v, vec!["b"]);
    // compress(["bcbc" + c for c in BASE[:26]] + ["bcb" + c for c in BASE if c != 'c'], accuracy=0.8)
    // ['bcb']
}

#[test]
fn test_gridpoints() {
    let (lat, lon) = (35.68952987243547, 139.69953972279566);
    assert_eq!(encode(lat, lon, 6), "xn774c");
    let bounds = to_bounds("xn774c");
    assert_eq!(
        bounds,
        Bounds {
            lat_min: 35.6890869140625,
            lon_min: 139.691162109375,
            lat_max: 35.694580078125,
            lon_max: 139.7021484375
        }
    );
    let gridsize = calc_gridsize(bounds.lon_max - bounds.lon_min, bounds.lat_max - bounds.lat_min, lat);
    assert_eq!(
        gridsize,
        GridSize {
            width: 993.3024217863972,
            height: 611.49622628141
        }
    );
    assert_eq!(split_bits("xn774c"), LatLonBits { lat: 22881, lon: 29099 });
    assert_eq!(
        calc_gridpoints(
            (lon - bounds.lon_min) / (bounds.lon_max - bounds.lon_min),
            (lat - bounds.lat_min) / (bounds.lat_max - bounds.lat_min),
            300.0,
            gridsize.width,
            gridsize.height
        ),
        vec![(0, -1), (0, 0), (1, -1), (1, 0)]
    );
    assert_eq!(create_circle(lat, lon, 300.0, 6), ["xn774b", "xn774c", "xn7750", "xn7751"]);
}
