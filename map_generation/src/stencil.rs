use image::io::Reader;
use image::{open, ImageBuffer, ImageFormat, Rgba, RgbaImage};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::ops::AddAssign;
use std::path::Path;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{mpsc, Arc, RwLock};
use std::thread::sleep;
use std::time::Duration;
use std::{thread, vec};
use tiff::decoder::{Decoder, DecodingResult};
use tiff::encoder::colortype::{ColorType, Gray32, RGB8};
use tiff::encoder::{self, TiffEncoder};
use tiff::tags::Tag;

use geo::point;
use geo::prelude::*;

use crate::common::{
    generate_gradient, get_cache_from_file, length, write_cache_to_file, ParMatrix,
};
use crate::Gradient;
const THREADS: usize = 48;
// Size of entire world
// const IMG_WIDTH: usize = 86400;
// const IMG_HEIGHT: usize = 36000;

// One tile
// const IMG_WIDTH: usize = 86400;
// const IMG_HEIGHT: usize = 36000;

// This will change for deployment
const TEST_IMAGE: &str = "archive/VNP46A2/Gap_Filled_DNB_BRDF-Corrected_NTL/2023/235/VNP46A2_A2023235_h19v04_001_2023243095926.tif";
const PIXEL_DIM: f64 = 0.004_166_666_666_666_667;
const MAX_DIST: usize = 3000;

pub fn stencil(img_width: usize, img_height: usize) -> Arc<ParMatrix> {
    //Load the tile
    let input_file = File::open(TEST_IMAGE).expect("Failed to open file");

    // Create an image reader with no memory limits
    let mut img_reader = Reader::new(BufReader::new(input_file));
    img_reader.no_limits();
    img_reader.set_format(ImageFormat::Tiff);

    let data_image = img_reader.decode().unwrap().to_luma16();

    //Get the dimension of the tile 2400
    // let dim = data_image.dimensions().0;

    let garstang_cache = Arc::new(get_or_create_garstang_cache());

    // Filter out the values in the image which are below the threshold
    // or values which are irrelevant
    let filtered = data_image
        .enumerate_pixels()
        .filter_map(|(x, y, c)| {
            //The value 10 was set to reduce noise
            if c[0] == u16::MAX || c[0] < 10 {
                return None;
            }
            Some((x, y, c[0]))
        })
        .collect::<Vec<(u32, u32, u16)>>();

    let filtered = Arc::new(filtered);

    // This type is being wrapped in two other types at the same time
    // Par matrix ensures interior mutability is possible
    // While ARC allows sharing of a refrence between threads.
    let result_image = Arc::new(ParMatrix::new(img_height, img_width));

    let dim_x = img_width as i32;
    let dim_y = img_height as i32;

    let mut joins = vec![];
    let current_working_index: Arc<RwLock<usize>> = Arc::new(RwLock::new(0));

    //Number of threads to spawn
    for _ in 0..THREADS {
        //Shared index such that each thread can iterate the counter to the next workload
        let cwi = current_working_index.clone();
        //These are the relevant pixels this is the copy of an Arc reference, not the actual data.
        let data = filtered.clone();

        // Value needed to for knowing when to stop
        let data_len = filtered.len();

        //Clone arc references
        let result_image_ref = result_image.clone();
        let garstang_cache_ref = garstang_cache.clone();

        joins.push(thread::spawn(move || loop {
            let tmp: (u32, u32, u16);

            if let Ok(mut i) = cwi.write() {
                if i.ge(&data_len) {
                    break;
                }

                tmp = data[*i];
                i.add_assign(1);
            } else {
                panic!("POISONED LOCK")
            }

            let (s_x, s_y, mut c) = tmp;
            // If the light value is too great, TODO remove this
            if c >= 20000 {
                c = 20000 - 1;
            }

            //Calculate projection distortion in x
            // let p1 = point!(
            //     x: offset_lat + (s_x as f64 * PIXEL_DIM),
            //     y: offset_lon + (s_y as f64 * PIXEL_DIM)
            // );
            // let p2 = point!(
            //     x: offset_lat + ((s_x + 1) as f64 * PIXEL_DIM),
            //     y: offset_lon + ((s_y) as f64 * PIXEL_DIM)
            // );
            // let delta_x = (p1.geodesic_distance(&p2) / 100.) as f32;

            // let p2 = point!(
            //     x: offset_lat + ((s_x) as f64 * PIXEL_DIM),
            //     y: offset_lon + ((s_y + 1) as f64 * PIXEL_DIM)
            // );
            // let delta_y = (p1.geodesic_distance(&p2) / 100.) as f32;

            // This calculates how wide and tall the stencil is
            // let stencil_size_x = (garstang_cache_ref[c as usize][3000] as f32 / delta_x) as i32;
            // let stencil_size_y = (garstang_cache_ref[c as usize][3000] as f32 / delta_y) as i32;
            let stencil_size_x = garstang_cache_ref[c as usize][3000] as i32;
            let stencil_size_y = stencil_size_x;

            let s_x = s_x as i32;
            let s_y = s_y as i32;

            // Generates a stencil by writing directly to atomic array
            for y_stencil_index in -stencil_size_y..stencil_size_y {
                for x_stencil_index in -stencil_size_x..stencil_size_x {
                    let dist = length(
                        // delta_x * x_stencil_index as f32,
                        // delta_y * y_stencil_index as f32,
                        x_stencil_index as f32,
                        y_stencil_index as f32,
                    );

                    //If the distance is greater than 299.9km
                    if dist > 2999. {
                        continue;
                    }
                    let mut dist = dist as usize;

                    //If the distance is less than 300m
                    //This only applies where we are calculating the center pixel
                    if dist < 3 {
                        dist = 3;
                    }
                    let x = s_x + x_stencil_index;
                    let y = s_y + y_stencil_index;

                    if x < 0 || x >= dim_x || y < 0 || y >= dim_y {
                        continue;
                    }

                    result_image_ref.write(
                        x as usize,
                        y as usize,
                        garstang_cache_ref[c as usize][dist],
                    );
                }
            }
            // if a % 1000 == 0 {
            //     println!("generating image {a}/{data_len}");
            // }
        }));
    }

    //Wait for all threads to finish
    for i in joins {
        _ = i.join();
    }

    result_image
}

fn get_image_dimensions() -> (usize, usize) {
    let file_path = Path::new(TEST_IMAGE);
    let file = File::open(file_path).expect("Cannot open file");

    let mut decoder = Decoder::new(file).expect("Cannot create TIFF decoder");
    let dimensions = decoder.dimensions().expect("Cannot read dimensions");

    (dimensions.0 as usize, dimensions.1 as usize)
}

pub fn generate_image_gray() {
    let (img_width, img_height) = get_image_dimensions();

    let result = stencil(img_width, img_height);

    let gradient = Arc::new(generate_gradient());

    let (tx, rx) = mpsc::channel::<(usize, Vec<u32>)>();
    let current_working_index: Arc<RwLock<usize>> = Arc::new(RwLock::new(0));
    let mut reader_threads = vec![];

    // receives finished results however it must wait for the correct ordered
    // result to finish before it can write it
    let writer_thread = thread::spawn(move || {
        let file = File::create("map_generation/tests/single_tile_test.tiff").unwrap();
        let w = BufWriter::new(file);

        let mut encoder = encoder::TiffEncoder::new_big(w).unwrap();
        let mut image = encoder
            .new_image::<Gray32>(img_width as u32, img_height as u32)
            .unwrap();
        image
            .encoder()
            .write_tag(Tag::Artist, "Image-tiff")
            .unwrap();

        image.rows_per_strip(1).unwrap();

        let mut strip_buffer = HashMap::new();
        let mut needed_row = 0;

        while let Ok((strip_row_number, strip)) = rx.recv() {
            if needed_row != strip_row_number {
                strip_buffer.insert(strip_row_number, strip);
            } else {
                image.write_strip(&strip).unwrap();
                needed_row += 1;
            }

            while let Some(buffer_strip) = strip_buffer.get(&needed_row) {
                image.write_strip(buffer_strip).unwrap();
                needed_row += 1;
            }
        }
        println!("I wrote {} lines", needed_row);
        image.finish().unwrap();
    });

    for _ in 0..THREADS - 1 {
        let cwi = current_working_index.clone();
        let ltx = tx.clone();
        let res = result.clone();
        let grad = gradient.clone();

        // Send results and what index it belongs to
        reader_threads.push(thread::spawn(move || loop {
            let index: usize;
            if let Ok(mut i) = cwi.write() {
                index = *i;
                if index >= img_height {
                    break;
                }
                i.add_assign(1);
            } else {
                panic!("POISONED LOCK")
            }

            let strip = res.read_row(index);

            ltx.send((index, strip)).unwrap();
        }));
    }

    for j in reader_threads {
        j.join().unwrap();
    }
    // Give reader thread a chance to drain the channel before we drop it.
    sleep(Duration::from_secs(2));
    // Drop the sending channel such that the reader knows nothing else will be recived.
    drop(tx);

    writer_thread.join().unwrap();
}

pub fn generate_image() {
    let (img_width, img_height) = get_image_dimensions();

    let result = stencil(img_width, img_height);

    let gradient = Arc::new(generate_gradient());

    let (tx, rx) = mpsc::channel::<(usize, Vec<u8>)>();
    let current_working_index: Arc<RwLock<usize>> = Arc::new(RwLock::new(0));
    let mut reader_threads = vec![];

    // receives finished results however it must wait for the correct ordered
    // result to finish before it can write it
    let writer_thread = thread::spawn(move || {
        let file = File::create("map_generation/tests/single_tile_test.tiff").unwrap();
        let w = BufWriter::new(file);

        let mut encoder = encoder::TiffEncoder::new_big(w).unwrap();
        let mut image = encoder
            .new_image::<RGB8>(img_width as u32, img_height as u32)
            .unwrap();
        image
            .encoder()
            .write_tag(Tag::Artist, "Image-tiff")
            .unwrap();

        image.rows_per_strip(1).unwrap();

        let mut strip_buffer = HashMap::new();
        let mut needed_row = 0;

        while let Ok((strip_row_number, strip)) = rx.recv() {
            if needed_row != strip_row_number {
                strip_buffer.insert(strip_row_number, strip);
            } else {
                image.write_strip(&strip).unwrap();
                needed_row += 1;
            }

            while let Some(buffer_strip) = strip_buffer.get(&needed_row) {
                image.write_strip(buffer_strip).unwrap();
                needed_row += 1;
            }
        }
        println!("I wrote {} lines", needed_row);
        image.finish().unwrap();
    });

    for _ in 0..THREADS - 1 {
        let cwi = current_working_index.clone();
        let ltx = tx.clone();
        let res = result.clone();
        let grad = gradient.clone();

        // Send results and what index it belongs to
        reader_threads.push(thread::spawn(move || loop {
            let index: usize;
            if let Ok(mut i) = cwi.write() {
                index = *i;
                if index >= img_height {
                    break;
                }
                i.add_assign(1);
            } else {
                panic!("POISONED LOCK")
            }

            let strip = convert_results_to_rgb(res.read_row(index), &grad);

            ltx.send((index, strip)).unwrap();
        }));
    }

    for j in reader_threads {
        j.join().unwrap();
    }
    // Give reader thread a chance to drain the channel before we drop it.
    sleep(Duration::from_secs(2));
    // Drop the sending channel such that the reader knows nothing else will be recived.
    drop(tx);

    writer_thread.join().unwrap();
}

// The input and output look the same, but the output is 3x longer
fn convert_results_to_rgb(result: Vec<u32>, gradient: &Gradient) -> Vec<u8> {
    let mut output = vec![];

    for i in result {
        let scaled = (i as f64).sqrt() / 355.;
        let color = gradient.sample_gradient(i);

        output.push(color.0);
        output.push(color.1);
        output.push(color.2);
    }

    output
}

fn get_or_create_garstang_cache() -> Vec<Vec<u32>> {
    let mut garstang_cache = vec![vec![0; MAX_DIST + 1]; 20_000];
    let dir_xy = 1. / (2. as f64).sqrt();
    let obs_dir: (f64, f64, f64) = (-dir_xy, 0., dir_xy);

    //check if cache file exists
    if let Some(cache_file) = get_cache_from_file() {
        for l in 0..garstang_cache.len() {
            for v in 0..garstang_cache[0].len() {
                garstang_cache[l][v] = cache_file[l][v];
            }
        }
        return garstang_cache;
    }

    for light in 0..20_000 {
        for dist in 0..MAX_DIST {
            // DNB VIIRS data is in nano watts per square cm
            // Garstang model uses total luminosity in lumen
            // Conversion from nW/cm^2 to l/m^2 is 0.000000001*683*100*100 = 0.00683
            // Total luminosity is then 0.00683*750*750 = 3841.875
            let lp = garstang::garstang_1989_calc(
                (light as f64) * 3841.875 / 0.15, // LP
                (dist as f64) / 10.,              // Distance in km from hecto meters
                0.,                               //H
                0.,                               //A
                obs_dir,                          //dir
            ) * 100000.; //Scale the output since we need the more significant figures and it will be saved as a u64.

            // If the value is below threshold set last index to the max relevant distance
            // This index will then be checked later in order to provide the size of the stencil.
            if lp < 1. {
                if light % 100 == 0 {
                    println!("source light: {light}, SETTING MAX INDEX: {dist} light_max: {} ,light_min: {},", garstang_cache[light][0], garstang_cache[light][dist]);
                }

                //The size of the stencil in pixels
                garstang_cache[light][MAX_DIST] = dist as u32;
                break;
            }
            garstang_cache[light][dist] = lp as u32;
        }
        if garstang_cache[light][MAX_DIST] == 0 && light % 10 == 0 {
            garstang_cache[light][MAX_DIST] = MAX_DIST as u32 - 1;
            println!(
                "source light: {light}, SETTING MAX INDEX: {MAX_DIST} light_max: {} ,light_min: {}",
                garstang_cache[light][0], garstang_cache[light][2999]
            );
        }
    }

    //Save the cache to file such that this calculation can be skipped in future
    write_cache_to_file(garstang_cache.clone());
    garstang_cache
}

// This should be done in paralell
// pub fn generate_image() {
//     create_and_save_empty_image()
//     let mut generated_image: RgbaImage = ImageBuffer::new(IMG_WIDTH, IMG_HEIGHT);

//     let gradient = generate_gradient();

//     let result = stencil();

//     for y in 0..IMG_WIDTH as usize {
//         for x in 0..IMG_HEIGHT as usize {
//             let scaled = (result.read(x, y) as f64).sqrt() / 355.;

//             let color = gradient.at(scaled).to_rgba8();
//             let mut alpha: u32 = (scaled * 5000.) as u32;
//             if alpha > 255 {
//                 alpha = 254;
//             }
//             generated_image.get_pixel_mut(x as u32, y as u32).0 =
//                 [color[0], color[1], color[2], alpha as u8];
//         }
//     }

//     generated_image
//         .save(format!("./map_generation/tests/uk_test1.tif"))
//         .unwrap();
// }
