
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

use crate::common::{generate_gradient, get_cache_from_file, write_cache_to_file, ParArray, length, length_square, get_image_raw_from_file, write_image_cache_to_file};

const TEST_IMAGE: &str = "/home/ardijan/repos/bachelor_thesis/light_pollution_map/sat_dl/archive/VNP46A2/Gap_Filled_DNB_BRDF-Corrected_NTL/2012/41/VNP46A2_A2012041_h17v03_001_2020039030351.tif";
const PIXEL_DIM :f64 = 0.004166666666666666609;



pub fn stencil()->Vec<Vec<u64>>{


    let max :f64= 0.0001;
    let scale = 1./max;



    let dir_xy = 1./(2. as f64).sqrt();
    let obs_dir :(f64,f64,f64) = (-dir_xy,0.,dir_xy);
    // let obs_dir :(f64,f64,f64) = (-0.1,0.,1.);


    let data_image = open(TEST_IMAGE).unwrap().to_luma16();
    let dim = data_image.dimensions().0;
    const MAX_DIST: usize = 3000;
    const MAX_DIST_SQUARED: usize = MAX_DIST*MAX_DIST;
    let mut garstang_cache = vec![vec![0;MAX_DIST+1];20_000];





    //Fill cache
    if let Some(cache_file) = get_cache_from_file(){

        for l in 0..garstang_cache.len() {
            for v in 0..garstang_cache[0].len(){
                garstang_cache[l][v] = cache_file[l][v];
            }
        }

    } else {
        for light in 0..20_000{
            for dist in 0..MAX_DIST{
                //Garstang model requires distance in km.

                // DNB VIIRS data is in nano watts per square cm
                // Garstang model uses total luminosity in lumen
                // Conversion from nW/cm^2 to l/m^2 is 0.000000001*683*100*100 = 0.00683
                // Total luminosity is then 0.00683*750*750 = 3841.875

                let lp = garstang::garstang_1989_calc( (light as f64)*3841.875, (dist as f64)/10.,0.,0., obs_dir)*1000000.;
                // let lp = garstang::garstang_1989_calc( (light as f64).sqrt(), (dist as f64)/10.,0.,0., obs_dir)*1000000000.;
                if light%100==0{
                // println!("source light: {light}, dist {dist}, lp_scaled: {lp}");
                }
                //Set last index to the max relevant distance

                if lp<1.{
                    if light%100==0{
                    println!("source light: {light}, SETTING MAX INDEX: {dist} light_max: {} ,light_min: {}", garstang_cache[light][0], garstang_cache[light][dist]);
                    }
                    garstang_cache[light][MAX_DIST]=dist as u32;
                    break;
                }
                garstang_cache[light][dist]=lp as u32;
            }
            if garstang_cache[light][MAX_DIST]==0 && light%10==0{
                garstang_cache[light][MAX_DIST]=MAX_DIST as u32-1;
                    println!("source light: {light}, SETTING MAX INDEX: {MAX_DIST} light_max: {} ,light_min: {}", garstang_cache[light][0], garstang_cache[light][2999]);
            }
        }
        write_cache_to_file(garstang_cache.clone());
    }

    let garstang_cache = Arc::new(garstang_cache);



    let filtered = data_image.enumerate_pixels().filter_map(|(x,y,c)|{
        //The value 10 was set to reduce noise
        if c[0] == u16::MAX || c[0]<1 {
            return None;
        }
        Some((x,y,c[0]))
    }).collect::<Vec<(u32,u32,u16)>>();


    let mut placeholder_matrix :Vec<Vec<AtomicU64>>= vec![];
    for _ in 0..2400{
        let mut line = vec![];
        for _ in 0..2400{
            line.push(AtomicU64::new(0))
        }
        placeholder_matrix.push(line);
    }





    println!("Placehoder size x{}   y{} ", placeholder_matrix.len(),placeholder_matrix[0].len());

    let image_matrix = Arc::new(ParArray{array:placeholder_matrix});

    let offset_lat: f64 = -10.;
    let offset_lon: f64 = 50.;

    // const CACHE_SIZE: usize = 500; //Goood one
    // const CACHE_SIZE: usize = 1000; //Testing
    // const MAX_RAD_PIXELS:i32 = CACHE_SIZE as i32*CACHE_SIZE as i32;

    // let mut distance_cache = [[0;CACHE_SIZE];CACHE_SIZE];
    let dim = dim as i32;
    println!("Max value {:?}", filtered.clone().into_iter().max_by(|a,b| { a.2.cmp(&b.2) } ));
    //filter out pixels with non important values.


    let mut joins = vec![];
    let current_working_index : Arc<RwLock<usize>>= Arc::new(RwLock::new(0));
    for _ in 0..24 {

        let cwi = current_working_index.clone();
        let data = filtered.clone();
        let data_len = filtered.len();

        let image_data_ref = image_matrix.clone();
        let garstang_cache_ref = garstang_cache.clone();

        joins.push(thread::spawn(move ||{
            loop {
                let mut tmp = (0,0,0);
                let mut a = 0;

                if let Ok(mut i) = cwi.write(){
                    if i.ge(&data_len) {
                        break;
                    }
                    a = i.clone();
                    tmp = data[i.clone()];
                    i.add_assign(1);

                } else {
                    panic!("UNABLE TO AQUIRE THIS VALUE")
                }



                let (s_x,s_y,c) = tmp;
                let mut delta = (0.,0.);
                let p1 = point!(x: offset_lat +(s_x as f64*PIXEL_DIM), y: offset_lon + (s_y as f64*PIXEL_DIM));
                let p2 = point!(x: offset_lat +((s_x+1) as f64 * PIXEL_DIM), y: offset_lon+ ((s_y) as f64*PIXEL_DIM));
                delta.0 = (p1.vincenty_distance(&p2).unwrap()/100.) as f32;

                let p1 = point!(x: offset_lat +(s_x as f64*PIXEL_DIM), y: offset_lon + (s_y as f64*PIXEL_DIM));
                let p2 = point!(x: offset_lat +((s_x) as f64 * PIXEL_DIM), y: offset_lon+ ((s_y+1) as f64*PIXEL_DIM));
                delta.1 = (p1.vincenty_distance(&p2).unwrap()/100.) as f32;

                let max_index = garstang_cache_ref[c as usize][3000] as i32;
                let s_x = s_x as i32;
                let s_y = s_y as i32;

                for y_stencil in -max_index..max_index{
                    for x_stencil in -max_index..max_index{

                        let dist = length_square(delta.0*x_stencil as f32,delta.1*y_stencil as f32).sqrt();
                        if dist>2999.{
                            continue;
                        }
                        let mut dist = dist as usize;
                        if dist<37{
                            dist = 37;
                        }
                        let x = s_x +x_stencil;
                        let y = s_y +y_stencil;

                        if x<0 || x>=dim || y<0 ||y>=dim { continue; }

                        image_data_ref.write(x as usize,y as usize, garstang_cache_ref[c as usize][dist] as u64);
                    }
                }
                if a%200==0{
                    println!("generating image {a}/{data_len}");
                }
            }

        }));

    }

    for i in joins{ i.join(); };

    let mut image = vec![vec![0;2400];2400];

    for y in 0..dim as usize{
        for x in 0..dim as usize{
            image[y][x]=image_matrix.read(x as usize,y as usize);
        }
    }
    // let image_matrix = image_matrix.;


    image
}


fn generate_image(){
    let mut generated_image: RgbaImage = ImageBuffer::new(2400,2400);

    let gradient = generate_gradient();

    let loaded_image;
    if let Some(cache_file) = get_image_raw_from_file(){
        loaded_image = cache_file;
    } else {
        loaded_image = stencil();
        write_image_cache_to_file(loaded_image.clone());
    }


    for y in 0..2400 as usize{
        for x in 0..2400 as usize{

            let scaled= (loaded_image[y][x]  as f64).sqrt()/250.;
            if scaled>0. && y%500==0{
                println!("scaled {scaled}");
            }
            let color = gradient.at(scaled).to_rgba8();
            let mut alpha :u32 = (scaled*5000.) as u32;
            if alpha>255{
                alpha=254;
            }
            generated_image.get_pixel_mut(x as u32, y as u32).0 = [color[0],color[1],color[2], alpha as u8];
        }
    }


    generated_image.save(format!("./map_generation/tests/uk_test.png")).unwrap();

}


pub fn version_3_threaded(){
    generate_image();
}
