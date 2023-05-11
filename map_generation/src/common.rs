use std::fs::File;
use std::io::{Read, Write};
use std::sync::atomic::{AtomicU64, Ordering};

use colorgrad::{Color, Gradient};

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
    let cache = serde_json::to_string(&cache).unwrap();

    let mut f = File::create(format!("./map_generation/cache/garstang_cache.json")).unwrap();
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

pub struct ParArray {
    pub array: Vec<Vec<AtomicU64>>,
}

unsafe impl Send for ParArray {}
unsafe impl Sync for ParArray {}

impl ParArray {
    pub fn write(&self, x: usize, y: usize, v: u64) {
        self.array[y][x].fetch_add(v, Ordering::Relaxed);
    }

    pub fn read(&self, x: usize, y: usize) -> u64 {
        self.array[y][x].fetch_max(0, Ordering::SeqCst)
    }
}

pub fn generate_gradient() -> Gradient {
    let black = (0, 0, 0);
    let gray = (100, 100, 100);
    let white = (255, 255, 255);
    let blue = (0, 0, 255);
    let blue_dark = (0, 0, 100);
    let green = (0, 255, 0);
    let green_dark = (0, 100, 0);
    let yellow = (255, 255, 0);
    let yellow = (255, 255, 100);
    let orange = (255, 150, 0);
    let red_dark = (150, 0, 0);
    let red = (255, 0, 0);
    let pink = (255, 150, 150);
    let dark_white = (180, 180, 180);
    let white = (255, 255, 255);

    let a = (110, 69, 0);
    let b = (207, 103, 0);
    let c = (255, 155, 56);
    let d = (255, 155, 56);
    let e = (250, 213, 112);
    let f = (250, 213, 112);

    colorgrad::CustomGradient::new()
        .colors(&[
            // Color::from(black),
            // Color::from(a),
            // Color::from(b),
            // Color::from(c),
            // Color::from(d),
            // Color::from(e),
            // Color::from(f),
            Color::from(gray),
            Color::from(blue_dark),
            Color::from(blue),
            Color::from(green_dark),
            Color::from(green),
            Color::from(orange),
            Color::from(yellow),
            Color::from(red_dark),
            Color::from(red),
            Color::from(pink),
            // Color::from(pink),
            Color::from(white),
            //Original
            // Color::from(black),
            // Color::from(gray),
            // Color::from(blue),
            // Color::from(green),
            // Color::from(yellow),
            // Color::from(orange),
            // Color::from(red),
            // Color::from(dark_white),
            // Color::from(white),
        ])
        .build()
        .unwrap()
}
