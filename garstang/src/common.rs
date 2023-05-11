use std::vec;

pub type Vector3D = (f64, f64, f64);
pub type Point3D = (f64, f64, f64);

// 10 to the power of p
pub fn ten_to_pow(p: i32) -> f64 {
    (10.0 as f64).powi(p)
}

// Secant function
pub fn sec(x: f64) -> f64 {
    1.0 / x.cos()
}

// e to the power of p
pub fn exp(p: f64) -> f64 {
    std::f64::consts::E.powf(p)
}

pub fn angle_between_vectors(a: Vector3D, b: Vector3D) -> f64 {
    (dot_product(a, b) / (length_of_vector(a) * length_of_vector(b)))
        .acos()
        .max(0.)
}

pub fn dot_product(a: Vector3D, b: Vector3D) -> f64 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

pub fn points_to_vector(start: Point3D, stop: Point3D) -> Vector3D {
    (stop.0 - start.0, stop.1 - start.1, stop.2 - start.2)
}

pub fn length_of_vector(a: Vector3D) -> f64 {
    (a.0.powi(2) + a.1.powi(2) + a.2.powi(2)).sqrt()
}

pub fn new_vector_normalized(vec: Vector3D) -> Vector3D {
    let len = length_of_vector(vec);
    (vec.0 / len, vec.1 / len, vec.2 / len)
}

pub fn displace_vector_or_point(a: Vector3D, b: Vector3D) -> Vector3D {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

pub fn scale_vector(a: Vector3D, s: f64) -> Vector3D {
    (a.0 * s, a.1 * s, a.2 * s)
}

//Convert degrees to radians
pub fn deg_to_rad(degree: f64) -> f64 {
    degree * std::f64::consts::PI / 180.
}

pub fn rad_to_deg(rad: f64) -> f64 {
    rad * 180. / std::f64::consts::PI
}

// Rotates vector about the y axis with the angle r
pub fn rotate_about_y_axis(vector: Vector3D, r: f64) -> Vector3D {
    let row_1: Vector3D = (r.cos(), 0.0, r.sin());
    let row_2: Vector3D = (0., 1., 0.);
    let row_3: Vector3D = (-r.sin(), 0.0, r.cos());

    (
        dot_product(row_1, vector),
        dot_product(row_2, vector),
        dot_product(row_3, vector),
    )
}

// Rotates vector about the y axis with the angle r
pub fn rotate_about_z_axis(vector: Vector3D, r: f64) -> Vector3D {
    let row_1: Vector3D = (r.cos(), -r.sin(), 0.);
    let row_2: Vector3D = (r.sin(), r.cos(), 0.);
    let row_3: Vector3D = (0., 0.0, 1.);

    (
        dot_product(row_1, vector),
        dot_product(row_2, vector),
        dot_product(row_3, vector),
    )
}
