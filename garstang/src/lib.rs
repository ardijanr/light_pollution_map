#![allow(non_camel_case_types)] // These are added because this project does not conform to rust's normal naming convention
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub mod common;
pub mod garstang_1986;
pub mod garstang_1989;

use crate::common::*;
pub use garstang_1986::garstang_1986_calc;
pub use garstang_1989::garstang_1989_calc;
// use tokio::task::JoinSet;



pub fn observer_view_rotated(
    degree_steps_y: f64,
    degree_steps_x: f64,
    distance_to_observer: f64,
) -> Vec<Vec<f64>> {
    let mut data = vec![];
    for b in 1..(90. / degree_steps_y) as usize {
        let mut row = vec![];
        for a in 1..(361. / degree_steps_x) as usize {
            let vec = rotate_about_z_axis((1., 0., 0.01), deg_to_rad(degree_steps_y * a as f64));
            let vec = rotate_about_y_axis(vec, deg_to_rad(degree_steps_x * b as f64));

            row.push(garstang_1989_calc(
                1000. * 3841.875 / 0.15,
                distance_to_observer,
                0.01,
                0.001,
                vec,
            ));
        }
        // break;

        data.push(row);
    }

    return data;
}

pub fn generate_image_from_2d_vec(data: Vec<Vec<f64>>, filename: String) {
    use image::{ImageBuffer, RgbImage};

    let dim_x = data[0].len();
    let dim_y = data.len();
    // Construct a new RGB ImageBuffer with the specified width and height.
    let mut img: RgbImage = ImageBuffer::new(dim_x as u32, dim_y as u32);


    for y in 0..dim_y {
        for x in 0..dim_x {
            let scaled_val = data[dim_y - y - 1][x] *3000. / 255. ;

            let r = (255.*scaled_val) as u8;
            let g = (169.*scaled_val) as u8;
            let b = (43.*scaled_val) as u8;
            img.get_pixel_mut(x as u32, y as u32).0 = [r, g, b];
        }
        // let scaled_val = (data[dim_y-y-1][x].log(10.)*100000.) as u8;
    }

    println!("FINISHED IMAGE {filename} dim_x:{dim_x}, dim_y: {dim_y}");

    img.save(format!("./video_frames/{filename}")).unwrap()
}


// #[tokio::main(flavor = "multi_thread", worker_threads = 16)]
// async fn main() {

//     // let distance = 0.75*(10 as f64);
//     // let file_name = format!("test_{}m.png",(distance*1000.) as u32);
//     // // let result = observer_view_yz_plane_polar(40.,10.,distance);
//     // let result = observer_view_rotated(1., 1., distance);
//     // generate_image_from_2d_vec(result,file_name);

//     let mut set = JoinSet::new();

//     for i in 1..800 {
//             // let result = observer_view_rotated(30.*4.,30.,distance);

//         set.spawn(async move {
//             let i = i.to_owned();
//             let distance = 0.05 * (i as f64);
//             let alignment = 5;
//             let file_name = format!("image_{:0alignment$}.png", i, alignment = alignment);

//             //Start from outside the city.
//             let result = observer_view_rotated(1., 1., distance+0.375);
//             generate_image_from_2d_vec(result, file_name);
//         });
//         //     // let result = observer_view_yz_plane_cart(30.*4.,30.,distance);
//     }

//     //Wait for downloads and geotiff generation to complete
//     while let Some(_) = set.join_next().await {}
// }