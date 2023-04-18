use crate::common::*;

// H is height above sea in km
// I_Up is in lumens per square cm
// Distance in km between city and observer
//
// obs_direction is the direction the observer is looking towards.
// this is a direction vector, meaning it is normalized so only its direction will be considered.
//
// The city has center at the origin and the observer has center at (distance,0,A)
//
pub fn garstang_1989_calc(LP: f64, distance: f64, H: f64,obs_direction:Vector3D) -> f64 {

    let N_m: f64 = 2.55 * ten_to_pow(19); // Particle density at sea level
    let σ_r: f64 = 4.6 * ten_to_pow(-27); //aerosol scattering coefficient
    const c: f64 = 0.104; // Molecular scale height in km^-1
    const π: f64 = std::f64::consts::PI;
    const A:f64 = 0.001; // A is height of observer above city
    const R: f64 = 0.325; // center from sides of a pixel or square city in km
    const K: f64 = 0.5; // Atmospheric clarity
    const S: f64 = 0.0; // Center of the earth
    const E: f64 = 0.0; // Radius of the earth sea level
    const F: f64 = 0.01; // Fraction of light being emitted upwards
    const G: f64 = 0.15;// Amount of light being reflected from the ground

    const gamma: f64 = 1.0 / 3.0; // Arbitrary value, don't change...
    let alpha = 0.657 + 0.059 * K;

    // let Ψ: f64 = 45.0; //Zenith angle of upward bound ray



    let dim_xy = 1;  // Split object into (dim * dim) points

    //Logic:
    // C_x-R_x + (2R_x/dim)*index-1 because index moves from 0 to dim-1;
    // C is the center point of the disk
    // in this case c is at origin of the coordinate system meaning
    let position_from_index = |index: usize| -> f64 {
        if dim_xy<=1 {
            return 0.;
        }
        -R + (2.0 * R / (dim_xy-1) as f64) * index as f64
    };

    let delta_theta = 0.0;
    let du = 0.0;


    // This is the direction the observer is looking towards as a unit vector.
    let directional_vector_OQ: Vector3D = new_vector_normalized(obs_direction);
    let up_vector: Vector3D = (0., 0., 1.);

    //R length along x axis at sea level
    let O: Point3D = (distance, 0., A);

    let dx = 2.0 * R / dim_xy as f64; //
    let dy = dx;
    let mut db = 0.0;

    for ix in 0..dim_xy {
        let pos_x = position_from_index(ix);

        for iy in 0..dim_xy {
            let pos_y = position_from_index(iy);
            let X: Vector3D = (pos_x, pos_y, 0.);

            let mut integration_index = 0;
            // let mut du = 30./1000.;
            loop {
                integration_index += 1;
                //Length of OQ
                let du = 0.0;
                let Q = 0.0;
                let Φ = 0.0;
                let θ = 0.0;
                let z = 0.0;
                let Ψ = 0.0; //Zenith angle XQ
                let s = 0.0;
                let h = 0.0;


                if du>100. || h>= 100. {
                    break;
                }


            }
        }
    }

    π * N_m * σ_r * exp(-c * H) * db
}