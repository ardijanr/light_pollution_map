use image::{open, ImageBuffer, RgbImage, RgbaImage};
use std::fs::File;
use std::io::{Read, Write};
use std::ops::{Add, AddAssign};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use std::{thread, vec};

use geo::point;
use geo::prelude::*;

use colorgrad::{Color, Gradient};

const TEST_IMAGE: &str = "./sat_dl/archive/VNP46A2/Gap_Filled_DNB_BRDF-Corrected_NTL/2012/41/VNP46A2_A2012041_h17v03_001_2020039030351.tif";
const PIXEL_DIM: f64 = 0.004166666666666666609;

pub mod common;
pub mod crawl;
pub mod stencil;

pub use crawl::*;
pub use stencil::*;

// const cutoff_distance:f64 = 100.; //Kilometers
// use garstang::garstang_1989_calc;

// Garstang cache first index is light source intensity
// the second index is the length which is in hecto meters
// The last value of the length array is the max distance at which
// the values get too small due to cutoff value

fn main() {
    generate_image();
    // version_4_crawler();
}


#[cfg(test)]
mod tests {
    use std::hash::Hash;

    use super::*;

    // #[test]
    // fn generate_stencil_test(){

    //     let mut stencils : HashMap<u16, String> = HashMap::new();
    //     let mut garstang_cache : HashMap<String, f64> = HashMap::new();
    //     let stencil = generate_stencil(10000, &mut stencils, &mut garstang_cache);

    //     //This slows down generation, but needed to create image.
    //     let mut stencil_image = vec![];
    //     for _ in -stencil_dim..stencil_dim{
    //         let mut stencil_line = vec![];
    //         for _ in -stencil_dim..stencil_dim{
    //             stencil_line.push(0.)
    //         }
    //         stencil_image.push(stencil_line)
    //     }

    //     for ((x,y),c) in stencil.0{
    //         println!("{x},{y}");
    //         stencil_image[y as usize][x as usize] = c;
    //     }
    //     generate_image_from_2d_vec(stencil_image, "test_stencil.png".to_string())

    // }
}

// pub fn generate_image_from_2d_vec(data : Vec<Vec<f64>>, filename: String){
//     use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};

//     let gradient = generate_gradient();

//     let dim_x = data[0].len();
//     let dim_y = data.len();
//     // Construct a new RGB ImageBuffer with the specified width and height.
//     let mut img: RgbImage = ImageBuffer::new(dim_x as u32, dim_y as u32);

//     // let mut imgbuf = image::ImageBuffer::new(dim_x, dim_y);
//     let max = data.iter().flatten().max_by(|a, b| a.total_cmp(b)).unwrap();
//     for y in 0..dim_y{

//         for x in 0..dim_x {
//             let data_val = data[y][x];
//             let scaled= data_val*1.;
//             let color = gradient.at(scaled).to_rgba8();
//             img.get_pixel_mut(x as u32, y as u32).0= [color[0],color[1],color[2]];
//         }

//     }

//     println!("FINISHED IMAGE {filename} dim_x:{dim_x}, dim_y: {dim_y}");

//     img.save(format!("./map_generation/tests/{filename}")).unwrap()
// }
