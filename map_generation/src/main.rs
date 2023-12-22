pub mod common;
pub mod gradient;
// pub mod crawl;
pub mod stencil;

// pub use crawl::*;
pub use stencil::*;
pub use gradient::*;

// use std::alloc::System;

// #[global_allocator]
// static GLOBAL: System = System;

fn main() {
    generate_image_gray();
}
