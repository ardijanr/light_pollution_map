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

use crate::common::{
    generate_gradient, get_cache_from_file, length, length_square_i32, write_cache_to_file,
    ParArray,
};

const TEST_IMAGE: &str = "/home/ardijan/repos/bachelor_thesis/light_pollution_map/sat_dl/archive/VNP46A2/Gap_Filled_DNB_BRDF-Corrected_NTL/2012/41/VNP46A2_A2012041_h17v03_001_2020039030351.tif";
const PIXEL_DIM: f64 = 0.004166666666666666609;

pub fn version_4_crawler() {
    let gradient = generate_gradient();

    let max: f64 = 0.0001;
    let scale = 1. / max;

    // let dir_xy = 1./(2. as f64).sqrt();
    // let obs_dir :(f64,f64,f64) = (-dir_xy,0.,dir_xy);
    let obs_dir: (f64, f64, f64) = (-0.1, 0., 1.);

    let data_image = open(TEST_IMAGE).unwrap().to_luma16();
    let dim = data_image.dimensions().0;
    const MAX_DIST: usize = 3000;
    const MAX_DIST_SQUARED: usize = MAX_DIST * MAX_DIST;
    let mut garstang_cache = vec![vec![0; MAX_DIST + 1]; 20_000];

    //Fill cache
    if let Some(cache_file) = get_cache_from_file() {
        for l in 0..garstang_cache.len() {
            for v in 0..garstang_cache[0].len() {
                garstang_cache[l][v] = cache_file[l][v];
            }
        }
    } else {
        for light in 0..20_000 {
            for dist in 1..MAX_DIST {
                //Garstang model requires distance in km.
                //Compensate for lumen
                let lp = garstang::garstang_1989_calc(
                    (light as f64) * 683. * 750. * 750.,
                    (dist as f64) / 10.,
                    0.,
                    0.,
                    obs_dir,
                ) * 1000.;
                if light % 100 == 0 {
                    // println!("source light: {light}, dist {dist}, lp_scaled: {lp}");
                }
                //Set last index to the max relevant distance

                if lp < 1. {
                    if light % 100 == 0 {
                        println!(
                            "source light: {}, SETTING MAX INDEX: {} KM, lp_scaled: {}",
                            light,
                            dist / 10,
                            lp
                        );
                    }
                    garstang_cache[light][MAX_DIST] = dist as u32;
                    break;
                }
                garstang_cache[light][dist] = lp as u32;
            }
            if garstang_cache[light][MAX_DIST] == 0 {
                garstang_cache[light][MAX_DIST] = MAX_DIST as u32 - 1;
                println!("source light: {light}, SETTING MAX INDEX: {MAX_DIST}");
            }
        }
        write_cache_to_file(garstang_cache.clone());
    }

    let garstang_cache = Arc::new(garstang_cache);

    let mut generated_image: RgbaImage = ImageBuffer::new(dim as u32, dim as u32);

    let mut source_image: Vec<Vec<u32>> = vec![vec![0; 2400]; 2400];

    for (x, y, c) in data_image.enumerate_pixels() {
        let l = c[0] as u32;
        if l < 100 && l > 0 {
            source_image[y as usize][x as usize] = l;
        }
    }

    let mut placeholder_matrix: Vec<Vec<AtomicU64>> = vec![];
    for _ in 0..2400 {
        let mut line = vec![];
        for _ in 0..2400 {
            line.push(AtomicU64::new(0))
        }
        placeholder_matrix.push(line);
    }

    // for y in 0..2400{
    //     for x in 0..2400{

    //         if source_image[y][x]>0{
    //             println!("YES THERE ARE VALUES");
    //         }
    //     }
    // }

    println!(
        "Placehoder size x{}   y{} ",
        placeholder_matrix.len(),
        placeholder_matrix[0].len()
    );

    let image_matrix = Arc::new(ParArray {
        array: placeholder_matrix,
    });

    let offset_lat: f64 = -10.;
    let offset_lon: f64 = 50.;

    // const CACHE_SIZE: usize = 500; //Goood one
    const CACHE_SIZE: usize = 200; //Testing
    const MAX_RAD_PIXELS: i32 = CACHE_SIZE as i32 * CACHE_SIZE as i32;

    // let mut distance_cache = [[0;CACHE_SIZE];CACHE_SIZE];
    let dim = dim as i32;
    // println!("Max value {:?}", filtered.clone().into_iter().max_by(|a,b| { a.2.cmp(&b.2) } ));
    //filter out pixels with non important values.

    let source_image = Arc::new(source_image);

    let mut joins = vec![];
    let current_working_index: Arc<RwLock<usize>> = Arc::new(RwLock::new(0));
    for _ in 0..12 {
        //This is along the y axis;
        let cwi = current_working_index.clone();

        let s_i = source_image.clone();
        let image_data_ref = image_matrix.clone();
        let garstang_cache_ref = garstang_cache.clone();
        let dim_local = 100;
        joins.push(thread::spawn(move || {
            let mut y;
            loop {
                if let Ok(mut i) = cwi.write() {
                    if i.ge(&(dim as usize)) {
                        // return;
                        break;
                    }
                    y = i.clone() as i32;
                    i.add_assign(1);
                } else {
                    panic!("UNABLE TO AQUIRE THIS VALUE")
                }

                let mut relevant_pixels = vec![];
                let mut delta = (0., 0.);

                for wx in 0..dim {
                    let x = wx - dim_local;
                    if x < 0 || x >= dim {
                        continue;
                    }

                    for _y in -dim_local..dim_local as i32 {
                        let actual_y = y - _y;
                        if actual_y < 0 || actual_y >= dim {
                            continue;
                        }

                        let p_val = s_i[actual_y as usize][wx as usize];
                        if p_val > 0 {
                            relevant_pixels.push((wx, y, p_val));
                            // println!("{wx},{_y}");
                        }
                    }

                    if relevant_pixels.len() == 0 {
                        continue;
                    }

                    let mut calc_offset = || {
                        let p1 = point!(
                            x: offset_lat + (x as f64 * PIXEL_DIM),
                            y: offset_lon + (y as f64 * PIXEL_DIM)
                        );
                        let p2 = point!(
                            x: offset_lat + ((x + 1) as f64 * PIXEL_DIM),
                            y: offset_lon + ((y) as f64 * PIXEL_DIM)
                        );
                        delta.0 = (p1.vincenty_distance(&p2).unwrap() / 100.) as f32;

                        let p1 = point!(
                            x: offset_lat + (x as f64 * PIXEL_DIM),
                            y: offset_lon + (y as f64 * PIXEL_DIM)
                        );
                        let p2 = point!(
                            x: offset_lat + ((x) as f64 * PIXEL_DIM),
                            y: offset_lon + ((y + 1) as f64 * PIXEL_DIM)
                        );
                        delta.1 = (p1.vincenty_distance(&p2).unwrap() / 100.) as f32;
                    };

                    if x % 200 == 0 || x == 0 {
                        calc_offset();
                    }

                    let mut sum = 0;
                    // println!("rel {}", relevant_pixels.len());

                    let mut popped = 0;
                    for index in 0..relevant_pixels.len() {
                        let (s_x, s_y, c) = relevant_pixels[index - popped];

                        let rel_dist = (s_x as i32 - x, s_y as i32 - y);

                        if length_square_i32(rel_dist.0, rel_dist.1) > (dim_local * dim_local) {
                            relevant_pixels.remove(index - popped);
                            popped += 1;
                            continue;
                        };

                        let dist =
                            length((delta.0 * rel_dist.0 as f32), (delta.1 * rel_dist.1 as f32))
                                as usize;
                        if dist >= 3000 {
                            continue;
                        }
                        // println!("x:{x}, y:{y} c:{c} dist:{dist}");

                        sum += garstang_cache_ref[c as usize][dist];
                    }

                    image_data_ref.write(x as usize, y as usize, sum as u64);
                }

                println!("Working on row {y}");
            }
        }));
    }

    for i in joins {
        i.join();
    }

    // let image_matrix = image_matrix.;

    for y in 0..dim as usize {
        for x in 0..dim as usize {
            let scaled = (image_matrix.read(x as usize, y as usize) as f64) / 10000.;
            let color = gradient.at(scaled).to_rgba8();
            let mut sum_color: u32 = (color[0] as u32 + color[1] as u32 + color[2] as u32) * 5;
            if sum_color > 255 {
                sum_color = 254;
            }

            generated_image.get_pixel_mut(x as u32, y as u32).0 =
                [color[0], color[1], color[2], sum_color as u8];
        }
    }

    generated_image
        .save(format!("./map_generation/tests/uk_test.png"))
        .unwrap();
}
