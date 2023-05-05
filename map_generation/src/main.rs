


use std::fs::{File};
use std::io::{Write, Read};
use std::ops::{Add, AddAssign};
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicU64, Ordering};
use std::{vec, thread};
use image::{open, RgbImage, ImageBuffer, RgbaImage};

use geo::prelude::*;
use geo::point;

use colorgrad::{Color, Gradient};

const TEST_IMAGE: &str = "/home/ardijan/repos/bachelor_thesis/light_pollution_map/sat_dl/archive/VNP46A2/Gap_Filled_DNB_BRDF-Corrected_NTL/2012/41/VNP46A2_A2012041_h17v03_001_2020039030351.tif";
const PIXEL_DIM :f64 = 0.004166666666666666609;


pub mod crawl;
pub mod stencil;
pub mod common;

pub use crawl::*;
pub use stencil::*;


// const cutoff_distance:f64 = 100.; //Kilometers
// use garstang::garstang_1989_calc;


// Garstang cache first index is light source intensity
// the second index is the length which is in hecto meters
// The last value of the length array is the max distance at which
// the values get too small due to cutoff value



fn main() {
    version_3_threaded();
    // version_4_crawler();
}







// // // A stencil is an image which shows how the garstang model drops of with distance around it.
// fn generate_stencil(light: u16,stencil_cache: &mut HashMap<u16, String>, garstang_cache: &mut HashMap<String, f64>)->(Vec<(PixelIndex,f64)>,String){
//     let dir_xy = 1./(2. as f64).sqrt();
//     let obs_dir :(f64,f64,f64) = (dir_xy,0.,dir_xy);

//     let mut stencil = vec![];
//         for y in -stencil_dim..stencil_dim {
//             for x in -stencil_dim..stencil_dim {

//                 let lat = x as f64*pixel_dim;
//                 let lon = y as f64*pixel_dim;

//                 let a = Location::new(0.,0.);
//                 let b = Location::new(lat, lon);
//                 //round of to 100 meters
//                 let mut distance = ((a.distance_to(&b).unwrap().meters()/100.) as u32 ) as f64 /10.;

//                 if x==0 && y==0 {
//                     distance = 0.375;
//                 }
//                 let key = format!("{},{}",distance,light);

//                 if let Some(val) = garstang_cache.get(&key){
//                     stencil.push((((x+stencil_dim) as u32,(y+stencil_dim) as u32),val.clone()));
//                 } else {
//                     let lp = garstang::garstang_1989_calc( light as f64, distance,0.,0., obs_dir);
//                     if lp<cutoff_value {
//                         continue;
//                     }
//                     stencil.push((((x+stencil_dim) as u32,(y+stencil_dim) as u32),lp));
//                     garstang_cache.insert(key, lp);
//                 }
//             }
//         }
//         let fname = format!("{light}.json");
//         stencil_cache.insert(light, fname.clone());
//     // }

//     (stencil,fname)

// }



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