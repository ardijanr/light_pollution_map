pub mod common;
// pub mod crawl;
pub mod stencil;

// pub use crawl::*;
pub use stencil::*;

// use std::alloc::System;

// #[global_allocator]
// static GLOBAL: System = System;

fn main() {
    generate_image_gray();
}
