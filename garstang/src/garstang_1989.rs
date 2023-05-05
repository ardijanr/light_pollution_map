use crate::common::*;

fn f_theta(_theta: f64) -> f64 {
    const π: f64 = std::f64::consts::PI;

    if 0.0 <= _theta && _theta <= deg_to_rad(10.0) {
        return 7.0 * exp(-0.2462 * _theta);
    }

    if deg_to_rad(10.0) < _theta && _theta <= π/2. {
        return 0.9124 * exp(-0.04245 * _theta);
    }
    if π/2. < _theta && _theta <= π {
        return 0.02;
    }

    return 0.02;
}

// LP is the total light output of the city
// Distance in km between city and observer
// H is height above sea in km
// A is the height of the observer above the city.
//
// obs_direction is the direction the observer is looking towards.
// this is a direction vector, meaning it is normalized so only its direction will be considered.
//
// The city has center at the origin and the observer has center at (distance,0,A)
//
pub fn garstang_1989_calc(LP: f64, mut distance: f64, H: f64,A:f64,obs_direction:Vector3D) -> f64 {

    let dbg_lvl = 0;

    if distance<0.325{
        distance = 0.325;
    }

    let N_m: f64 = 2.55 * ten_to_pow(19); // Particle density at sea level
    let σ_r: f64 = 4.6 * ten_to_pow(-27); //aerosol scattering coefficient
    const c: f64 = 0.104; // Molecular scale height in km^-1
    const π: f64 = std::f64::consts::PI;
    // const A:f64 = 0.00; // A is height of observer above city
    const R: f64 = 0.325; // center from sides of a pixel or square city in km
    const K: f64 = 0.5; // Atmospheric clarity
    const E: f64 = 6371.; // Radius of the earth sea level
    const F: f64 = 0.15; // Fraction of light being emitted upwards
    const G: f64 = 0.15; // Amount of light being reflected from the ground
    const gamma: f64 = 1.0 / 3.0; // Arbitrary value, don't change...
    const ε: f64 = 16./(9.*π);

    let a = 0.657 + 0.059 * K;

    //Defines the center of the earth, with the added height of the system.
    let SR = E+H;
    let S: Point3D = (0.,0.,-SR);
    // The radius of the system

    let mut dim_x = 1;  // Split object into (dim * dim) points
    let mut dim_y = 1;  // Split object into (dim * dim) points

    //Logic:
    // C_x-R_x + (2R_x/dim)*index-1 because index moves from 0 to dim-1;
    // C is the center point of the disk
    // in this case c is at origin of the coordinate system meaning
    let position_from_index = |index: usize,dim:usize| -> f64 {
        if dim<=1 {
            return 0.;
        }
        -R + (2.0 * R / (dim-1) as f64) * index as f64
    };

    let delta_θ = 0.0174533*2.;


    //The arc angle supsended by the distance along the earth D+H, where H is the height above sea level
    let OS_angle = distance/E;
    // This is the direction the observer is looking towards as a unit vector rotated to fit the global coordinate system
    // let dir_vec_OQ_norm = new_vector_normalized(obs_direction);
    let norm_dir = new_vector_normalized(obs_direction);
    let dir_vec_OQ_norm: Vector3D = rotate_about_y_axis(norm_dir,OS_angle);
    let up_vector: Vector3D = (0., 0., 1.);

    //R length along x axis at sea level
    let O: Point3D = ((SR+A)*(distance/SR).sin(),0.,(SR+A)*(distance/SR).cos()-SR);
    // let O: Point3D = (E*(distance/E).sin(),0.,(distance/E).cos()*E-E);

    let dx = 2.0 * R / dim_x as f64; //
    let dy = 2.0 * R / dim_y as f64;





    let mut sum = 0.0;

    let mut dbg_text = "".to_string();



    if dbg_lvl>=1{
            println!("");
            println!("----------------DEBUG-LEVEL-1--------------");
            println!("norm_dir: {norm_dir:?}, dir_rotated: {dir_vec_OQ_norm:?}, S: {S:?} , OS_angle: {OS_angle}, SR: {SR}, , dx: {dx}, dy: {dy} ")
    }

    if dir_vec_OQ_norm.2<0.{
        // println!("Negative Z, skipping {dir_vec_OQ_norm:?}");
        return 0.;
    }

    for ix in 0..dim_x {
        let pos_x = position_from_index(ix,dim_x);

        for iy in 0..dim_y {
            let pos_y = position_from_index(iy,dim_y);
            let X: Point3D = (pos_x, pos_y, 0.);

            let XO = points_to_vector(X,O);
            let θ = angle_between_vectors(points_to_vector(O, X), dir_vec_OQ_norm);

            // This is the point where the integration starts since its the where the line starts as "seen" from the perspective of the city.
            // Meaning its where the illumination of the line which is observed along starts.
            let Φ_start = angle_between_vectors((XO.0,0.,XO.2), (1.,0.,0.));
            // println!("phi_start: {}",Φ_start);
            // println!("phi_start: {}, XO: {:?}, O: {:?}", rad_to_deg( Φ_start),XO,O);
            let mut integration_index = 0;

            // Starting distance from O to Q for the integration.
            let mut u_prev = distance * Φ_start.sin()/(π-Φ_start-θ).sin();


            if dbg_lvl >= 2 {
                let theta = rad_to_deg(θ);
                let phi_start = rad_to_deg(Φ_start);

                println!("----------------DEBUG-LEVEL-2--------------");
                println!("X: {X:?}, XO: {XO:?} , θ: {theta} , Φ_start: {phi_start}, u_prev: {u_prev} ")
            }
            let mut saved_p1=0.;
            loop {
                integration_index += 1; //Not used after this point

                let Φ = Φ_start + delta_θ * integration_index as f64; //+ 1 degree in radians

                let XQO_angle = π - Φ - θ;

                //This is if the angle Φ is getting too big.
                if XQO_angle < delta_θ || XQO_angle > π-delta_θ {
                    break;
                }

                //Calculate the distance to the point Q
                let u = distance * Φ.sin()/(XQO_angle).sin();
                let du = u - u_prev;

                let OQ_vec = scale_vector(dir_vec_OQ_norm, u);
                let Q : Point3D = displace_vector_or_point(O,OQ_vec );

                let XQ_vec = points_to_vector(X, Q);


                let z = angle_between_vectors(OQ_vec,up_vector); //Zenith angle OQ
                let Ψ = angle_between_vectors(XQ_vec, up_vector); //Zenith angle for XQ
                let s = length_of_vector(XQ_vec);
                let SQ = points_to_vector(S, Q);
                let h = length_of_vector(SQ)-SR;

                if h>100. {
                    break;
                }

                //LOCALLY SCOPED FUNCTIONS
                let I_up = ||{
                    LP/(2.*4.) * (2.*G*(1.-F)*Ψ.cos() + 0.554 * F * Ψ.powi(4))
                };

                let p = |A_local:f64,angle:f64, c_:f64,dist_:f64|->f64{
                    c.powi(-1)*exp(-c_*A_local)*sec(angle)*(
                    1. - exp(-c_*dist_*angle.cos())
                    + (ε * angle.tan().powi(2))/(2. * c_ * SR)*(
                            (c_.powi(2) * dist_.powi(2) * angle.cos().powi(2) + 2.*c_*dist_*angle.cos() + 2.)*exp(-c_*dist_*angle.cos()) - 2.
                        )
                    )
                };


                let mut extinction_fuction = | A_local:f64, angle:f64, dist_a:f64, is_OQ:bool |->f64{
                    let p1;
                    //If we have passed the cuttof point.
                    if XQO_angle<π/2. && is_OQ{
                        p1 = saved_p1;
                    } else {
                        p1 = p(A_local,angle,c,dist_a) ;
                        saved_p1 = p1;
                    }

                    //P1 is the same as P2 with a replacing c everywhere, an internal c_ is therefore used

                    return exp(-N_m*σ_r*exp(-c*H)*( p1+11.778*K*p(A_local,angle,a,dist_a) ) );
                };

                let I_up_val = I_up();
                let EF_QO_val = extinction_fuction(A,z,u,true);
                let EF_XQ_val = extinction_fuction(0.,Ψ,s,false);

                let DS_val = 1. + (gamma*p(0.,Ψ, c,s) + 11.11* K*p(0.,Ψ, a,s))*N_m*σ_r*exp(-c*H);

                let rest_a = exp(-c * h); // * 3. * ((1. + (θ + Φ).cos().powi(2)) / (16. * π))
                let rest_b = 3.*(1. + (θ + Φ).cos().powi(2));
                let rest_c = 16.0*π;
                let rest_d = exp(-a*h)*11.11*K;
                if rest_d>1000.{
                    println!("EXCESSIVE");
                    // panic!()
                }
                let rest_e = f_theta(θ + Φ);
                let rest = (rest_a*rest_b/rest_c)+rest_d*rest_e;



                let new_db = I_up_val
                    *s.powi(-2)
                    * EF_QO_val
                    * EF_XQ_val
                    * DS_val
                    * rest
                    * du // Contribution along du
                    * (dx * dy / 4.*R.powi(2)); // (2*R)^2 infinitesimal area / actual area = contribution weight

                sum += new_db;



                if dbg_lvl == 3 {
                    let phi = rad_to_deg(Φ);
                    let XQO_ANGLE = rad_to_deg(XQO_angle);
                    let psi = rad_to_deg(Ψ);
                    let z = rad_to_deg(z);

                    println!("----------------DEBUG-LEVEL-3--------------");
                    println!("db: {sum},index: {integration_index},  new_db: {new_db}, Q: {Q:?}, s:{s}, u:{u}, h:{h}, du: {du}, phi: {phi}, XQO_ANGLE: {XQO_ANGLE}, psi: {psi}, z: {z}, I_up_val: {I_up_val}, EF_QO_val: {EF_QO_val}, EF_XQ_val: {EF_XQ_val}, DS_val: {DS_val}, rest_a: {rest_a}, rest_b: {rest_b}, rest_c: {rest_c}, rest_d: {rest_d}, rest_e: {rest_e}, rest: {rest} ");
                }

                u_prev = u; //Store variable for next integration
            }
        }
    }
    // println!("sum: {sum}");
    let b = π * N_m * σ_r * exp(-c * H) * sum;
    //Visual range
    b
    // 2.171*(1.+0.1122 * b.sqrt()).ln()
    // if b> 1479. {
    //     return 7.93 + 2.171*(1.+0.1122 * b.sqrt()).ln()
    // } else {
    //     return 4.305 + 2.171*(1.+0.001122 * b.sqrt()).ln()
    // }
}