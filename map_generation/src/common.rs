use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};

use crate::{gradient, Gradient};

pub type PixelIndex = (u32, u32);

pub fn length(a: f32, b: f32) -> f32 {
    (a.powi(2) + b.powi(2)).sqrt()
}

pub fn length_square(a: f32, b: f32) -> f32 {
    a.powi(2) + b.powi(2)
}

pub fn length_square_i32(a: i32, b: i32) -> i32 {
    a * a + b * b
}
pub fn distance_squared(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0) * (a.0 - b.0) + (a.1 - b.1) * (a.1 - b.1)
}

pub fn write_stencil_to_file(stencil: Vec<(PixelIndex, f64)>, fname: String) {
    let stencil = serde_json::to_string(&stencil).unwrap();

    let mut f = File::create(format!("./map_generation/cache/stencil_cache/{fname}")).unwrap();
    f.write_all(stencil.as_bytes()).unwrap();
}

pub fn get_stencil_from_file(fname: String) -> Vec<(PixelIndex, f64)> {
    let mut file = File::open(format!("./map_generation/cache/stencil_cache/{fname}")).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    serde_json::from_str(&contents).unwrap()
}

pub fn write_cache_to_file(cache: Vec<Vec<u32>>) {
    let cache_location = "./map_generation/cache";

    if !Path::new(cache_location).exists() {
        match fs::create_dir(&cache_location) {
            Ok(_) => (),
            Err(e) => println!("Error creating folder: {}", e),
        }
    }

    let cache = serde_json::to_string(&cache).unwrap();

    let mut f = File::create(format!("{cache_location}/garstang_cache.json")).unwrap();
    f.write_all(cache.as_bytes()).unwrap();
}

pub fn get_cache_from_file() -> Option<Vec<Vec<u32>>> {
    let mut file = File::open(format!("./map_generation/cache/garstang_cache.json")).ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok()?;

    let de: Vec<Vec<u32>> = serde_json::from_str(&contents).unwrap();
    Some(de)
}

pub fn write_image_cache_to_file(cache: Vec<Vec<u64>>) {
    let cache = serde_json::to_string(&cache).unwrap();

    let mut f = File::create(format!("./map_generation/cache/cached_image.json")).unwrap();
    f.write_all(cache.as_bytes()).unwrap();
}

pub fn get_image_raw_from_file() -> Option<Vec<Vec<u64>>> {
    let mut file = File::open(format!("./map_generation/cache/cached_image.json")).ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok()?;

    let de: Vec<Vec<u64>> = serde_json::from_str(&contents).unwrap();
    Some(de)
}

// Struct the unsafe impl is to tell the compiler "trust me"
// It is fine in this case because we can guarantee that it
// is thread safe due because all operations are performed
// on atomic types
pub struct ParMatrix {
    inner: Vec<Vec<AtomicU32>>,
}

unsafe impl Send for ParMatrix {}
unsafe impl Sync for ParMatrix {}

impl ParMatrix {
    pub fn new(size_y: usize, size_x: usize) -> ParMatrix {
        let mut matrix: Vec<Vec<AtomicU32>> = vec![];
        for _ in 0..size_y {
            let mut line = vec![];
            for _ in 0..size_x {
                line.push(AtomicU32::new(0))
            }
            matrix.push(line);
        }

        ParMatrix { inner: matrix }
    }

    pub fn write(&self, x: usize, y: usize, v: u32) {
        self.inner[y][x].fetch_add(v, Ordering::Relaxed);
    }

    pub fn read(&self, x: usize, y: usize) -> u32 {
        self.inner[y][x].fetch_max(0, Ordering::SeqCst)
    }

    pub fn read_row(&self, y: usize) -> Vec<u32> {
        let width = self.inner[0].len();
        let mut res = vec![0; width];
        for x in 0..width {
            res[x] = self.inner[y][x].fetch_max(0, Ordering::SeqCst);
        }

        res
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.inner.len(), self.inner[0].len())
    }
}

pub fn generate_gradient() -> Gradient {
    let mut gradient = Gradient::new((0, 0, 0, 255), (0, 0, 0, 255));
    let colors = [
        (64, 64, 64, 255), //1.74
        (128, 128, 128, 255),
        (22, 54, 92, 255),
        (54, 96, 146, 255),
        (83, 141, 213, 255),
        (79, 98, 40, 255),
        (118, 147, 60, 255),
        (255, 255, 0, 255),
        (255, 192, 0, 255),
        (255, 0, 0, 255),
        (192, 80, 77, 255),
        (218, 150, 148, 255),
        (255, 255, 255, 255),
    ];

    let mut value = 1.74;
    for i in 0..colors.len() {
        value = 2. * value;

        gradient.add_indexed_color((value * 100.) as u32, colors[i])
    }

    gradient
}
