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
pub fn garstang_1986_calc(LP: f64, distance: f64, H: f64, obs_direction: Vector3D) -> f64 {
    let N_m: f64 = 2.55 * ten_to_pow(19); // Particle density at sea level
    let σ_r: f64 = 4.6 * ten_to_pow(-27); //aerosol scattering coefficient
    const c: f64 = 0.104; // Molecular scale height in km^-1
    const π: f64 = std::f64::consts::PI;
    const A: f64 = 0.002; // A is height of observer above city
    const R: f64 = 0.325; // center from sides of a pixel or square city in km
    const K: f64 = 0.1; // Atmospheric clarity
    const gamma: f64 = 1.0 / 3.0; // Arbitrary value, don't change...
    let a = 0.657 + 0.059 * K;
    const F: f64 = 0.1; // Fraction of light being emitted upwards
    const G: f64 = 0.15; // Amount of light being reflected from the ground

    // let Ψ: f64 = 45.0; //Zenith angle of upward bound ray

    fn f_theta(_theta: f64) -> f64 {
        if 0.0 <= _theta && _theta <= deg_to_rad(10.0) {
            return 7.0 * exp(-0.2462 * _theta);
        }

        if deg_to_rad(10.0) < _theta && _theta <= π / 2. {
            return 0.9124 * exp(-0.04245 * _theta);
        }
        if π / 2. < _theta && _theta <= π {
            return 0.02;
        }

        return 0.02;
    }

    let dim = 6; // Split object into (dim * dim) points

    //Logic:
    // C_x-R_x + (2R_x/dim)*index-1 because index moves from 0 to dim-1;
    // C is the center point of the disk
    // in this case c is at origin of the coordinate system meaning
    let position_from_index = |index: usize| -> f64 {
        if dim <= 1 {
            return 0.;
        }
        -R + (2.0 * R / (dim - 1) as f64) * index as f64
    };

    let du = 50. / 1000.; //in km

    //Dynamically calculate DU based on angle between QX and QO

    // This is the direction the observer is looking towards as a unit vector.
    let directional_vector_OQ: Vector3D = new_vector_normalized(obs_direction);
    let up_vector: Vector3D = (0., 0., 1.);

    //R length along x axis at sea level
    let O: Vector3D = (distance, 0., A);
    let dx = 2.0 * R / dim as f64; //
    let dy = dx;
    let mut db = 0.0;

    for ix in 0..dim {
        let pos_x = position_from_index(ix);

        for iy in 0..dim {
            let pos_y = position_from_index(iy);
            let X: Vector3D = (pos_x, pos_y, 0.);

            let mut iu = 0;
            // let mut du = 30./1000.;
            loop {
                iu += 1;
                //Length of OQ
                let uu = du * iu as f64;
                let Q = displace_vector_or_point(O, scale_vector(directional_vector_OQ, uu));
                let OQ_vec = points_to_vector(O, Q);
                let XQ_vec = points_to_vector(X, Q);

                let Φ = angle_between_vectors(XQ_vec, points_to_vector(X, O));
                let θ = angle_between_vectors(OQ_vec, points_to_vector(O, X));
                let z = angle_between_vectors(up_vector, OQ_vec);
                let Ψ = angle_between_vectors(up_vector, XQ_vec);
                let s = length_of_vector(XQ_vec);
                let h = Q.2.max(0.);

                if uu > 100. || h >= 100. {
                    break;
                }

                //LOCALLY SCOPED FUNCTIONS
                let I_up = || {
                    LP / (2. * 4.)
                        * (2. * G * (1. - F) * Ψ.cos() + 0.554 * F * rad_to_deg(Ψ).powi(4))
                };

                fn extinction_function(
                    start: Point3D,
                    end: Point3D,
                    a_local: f64,
                    h: f64,
                    A_local: f64,
                    N_m: f64,
                    σ_r: f64,
                    H: f64,
                    angle: f64,
                ) -> f64 {
                    let p = || -> f64 {
                        ((exp(-c * A_local) - exp(-c * h)) / c)
                            + (11.778 * K * 1. / a_local
                                * (exp(-a_local * A_local) - exp(-a_local * h)))
                    };

                    exp(-N_m * σ_r * exp(-c * H) * p() * sec(angle))
                };

                // Double scattering function
                let double_scattering = || -> f64 {
                    let N_a_sigma_a = 11.11 * K * N_m * σ_r * exp(-c * H);

                    1.0 + N_a_sigma_a * (1. - exp(-a * s * Ψ.cos())) / (a * Ψ.cos())
                        + (gamma * N_m * σ_r * exp(-c * H) * (1. - exp(-c * s * Ψ.cos()))
                            / (c * Ψ.cos()))
                            / (c * Ψ.cos())
                };

                let frac_s = s.powi(-2);
                let ef_xq = extinction_function(X, Q, a, h, 0., N_m, σ_r, H, Ψ);
                let ef_qo = extinction_function(Q, O, a, h, O.2, N_m, σ_r, H, z);
                // let rest = (exp(-c * h) * 3. * ((1. + (θ + Φ).cos().powi(2)) / (16. * π)) + exp(-alpha * h) * 11.11 * K * f_theta(θ + Φ));

                let _d_scattering = double_scattering();
                let rest_a = exp(-c * h); // * 3. * ((1. + (θ + Φ).cos().powi(2)) / (16. * π))
                let rest_b = 3. * (1. + (θ + Φ).cos().powi(2));
                let rest_c = 16.0 * π;
                let rest_d = exp(-a * h) * 11.11 * K;
                let rest_e = f_theta(θ + Φ);
                let rest = (rest_a * rest_b / rest_c) + rest_d * rest_e;

                let new_db = I_up()
                    * frac_s
                    * ef_xq
                    * ef_qo
                    * _d_scattering
                    * rest
                    * du // Contribution along du
                    * (dx * dy / 4.*R.powi(2)); // (2*R)^2 infinitesimal area / actual area = contribution weight
                db += new_db;

                if db.is_nan() {
                    println!("DB is NAN");
                    let __a = exp(-c * h);
                    let __b = (1. + (θ + Φ).cos().powi(2)) / (16. * π);
                    let __c = exp(-a * h);
                    let __d = K * f_theta(θ + Φ);
                }

                // let Φ = rad_to_deg(Φ);
                // let θ = rad_to_deg(θ);
                // let z = rad_to_deg(z);
                // let Ψ = rad_to_deg(Ψ);
                // println!("iu= {iu}, X={X:?}, O={O:?}, Q={Q:?}, XQ_vec={XQ_vec:?}, OQ_vec={OQ_vec:?} ,Φ={Φ}, θ={θ} z={z}, Ψ={Ψ}, s={s}, _d_scattering={h}, rest={rest}, du={du}, new_db={new_db}, I_up={I_up}");
                // println!("iu={iu}, db={db}, s={s}, new_db={new_db}, h={h} ,u={uu}, du={du} frac_s={frac_s}, ef_xq={ef_xq}, ef_qo={ef_qo}, _d_scattering={_d_scattering}, rest={rest}, du={du}, I_up={I_up}");
            }
        }
    }

    π * N_m * σ_r * exp(-c * H) * db
}
