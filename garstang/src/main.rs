pub mod garstang_1986;
pub mod garstang_1989;
pub mod common;

pub use garstang_1986::garstang_1986_calc;
pub use garstang_1989::garstang_1989_calc;
use crate::common::*;

fn main() {

    // garstang_1989_calc(1000., 5., 0.,0., (1.,0.,0.));

        let distance = 0.75*(2 as f64);
        let file_name = format!("test_{}m.png",(distance*1000.) as u32);
        let result = observer_view_yz_plane_polar(10.*4.,10.,distance);
        generate_image_from_2d_vec(result,file_name);


    // for i in 1..11 {
    //     let distance = 0.75*(2 as f64);
    //     let file_name = format!("test_{}m.png",(distance*1000.) as u32);
    // //     // let result = observer_view_rotated(30.*4.,30.,distance);
    //     let result = observer_view_yz_plane_polar(10.*4.,10.,distance);
    // //     // let result = observer_view_yz_plane_cart(30.*4.,30.,distance);
    //     generate_image_from_2d_vec(result,file_name);
    // }
}



fn observer_view_yz_plane_cart(width_y:f64,height_z:f64,distance_to_observer:f64)->Vec<Vec<f64>>{
    //calculate values for the plane the observer is looking at facing the city.

    let pixel_per_km = 2;
    let dim_y = width_y as u32 *pixel_per_km;
    let dim_z = height_z as u32 *pixel_per_km;
    // let observer: Vector3D = (distance_x,0.,0.);
    // let dir_vec: Vector3D= (0.,y,z)-observer;

    let mut yz_plane = vec![];

    for iz in 0..dim_z {
        let z = (height_z/dim_z as f64)*iz as f64 + 0.1;
        // println!("height={z} km");

        let mut y_line = vec![];
        for iy in 0..dim_y {
            let mut y = 0.;
            if dim_y>1{
                y = (-width_y/2.)+(width_y/(dim_y-1) as f64)*(iy as f64);
            }


            //This is the same as P - O if p is pixel position and O is observer
            let dir_vec : Vector3D = (-distance_to_observer,y,z);


            y_line.push(garstang_1986_calc(1000., distance_to_observer, 0.01,  dir_vec));
            // y_line.push(garstang_1989_calc(1000., distance_to_observer, 0.01, 0.001, dir_vec));
            println!("X{iy},y:{iz}");

            //Generate image

        }

        yz_plane.push(y_line);
    }

    // dbg!(yz_plane);
    yz_plane
}


fn observer_view_yz_plane_polar(width_y:f64,height_z:f64,distance_to_observer:f64)->Vec<Vec<f64>>{
    //calculate values for the plane the observer is looking at facing the city.

    let pixel_per_km = 10;
    let dim_y = width_y as u32 *pixel_per_km;
    let dim_z = height_z as u32 *pixel_per_km;
    let pi = std::f64::consts::PI;
    let mut yz_plane = vec![];

    for iz in 1..dim_z+1 {

        let elevation = (pi/(2.*dim_z as f64))*(iz as f64);
        let mut y_line = vec![];
        for iy in 0..dim_y {
            let azimuth = (2.*pi/dim_y as f64)*iy as f64;


            let dir_vec :Vector3D= (elevation.cos()*azimuth.cos(),elevation.cos()*azimuth.sin(),elevation.sin());
            // println!("{:?}",dir_vec);

            y_line.push(garstang_1989_calc(1000., distance_to_observer, 0.,0., dir_vec));

        }
        // println!("FINISHED HORIZONTAL LINE");
        yz_plane.push(y_line);
    }

    yz_plane
}



fn observer_view_rotated(width_y:f64,width_x:f64,distance_to_observer:f64)->Vec<Vec<f64>>{

    let mut data = vec![];
    for b in 1..90{
        let mut row = vec![];
        for a in 1..360{
            let vec = rotate_about_z_axis((-1.,0.,0.), deg_to_rad(a as f64));
            // println!("z_rot: {vec:?}, angle: {b}");
            let vec = rotate_about_y_axis(vec, deg_to_rad(b as f64));
            // println!("y_rot: {vec:?}, angle: {b}");

            row.push(garstang_1989_calc(1000., distance_to_observer, 0.,0., vec));

        }
        // break;

        data.push(row);
    }

    return data;
}
//calculate triangele stuff to make image

//

fn generate_image_from_2d_vec(data : Vec<Vec<f64>>,filename: String){
    use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};

    let dim_x = data[0].len();
    let dim_y = data.len();
    // Construct a new RGB ImageBuffer with the specified width and height.
    let mut img: RgbImage = ImageBuffer::new(dim_x as u32, dim_y as u32);

    // let mut imgbuf = image::ImageBuffer::new(dim_x, dim_y);
    let max = data.iter().flatten().max_by(|a, b| a.total_cmp(b)).unwrap();
    for y in 0..dim_y{

        for x in 0..dim_x {
            // let pixel = image::Rgb<u8>::from_slice ;
            // let data_val = data[y][x];
            let scaled_val = (data[dim_y-y-1][x]*(1./max )*255.) as u8;
            // println!("{x},{y}, scaled_val= {scaled_val}, data_val={data_val}"); 
            img.get_pixel_mut(x as u32, y as u32).0= [0,scaled_val,0];
        }
        // let scaled_val = (data[dim_y-y-1][x].log(10.)*100000.) as u8;

    }

    println!("FINISHED IMAGE {filename} dim_x:{dim_x}, dim_y: {dim_y}");

    img.save(format!("./v2_observers_perspective/{filename}")).unwrap()
}