#[macro_use]
mod vector;
mod material;
mod types;
mod ray;

use crate::types::{Float, HittableList, Sphere, Camera, Frame, drand48, Pixel};
use crate::vector::Vec3;
use crate::material::{Lambertian, Metal, Dielectric};
use image::ColorType;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread::{sleep, spawn, Thread};
use std::time::Duration;
use std::sync::mpsc::{channel, Sender, Receiver};

pub const ASPECT_RATIO:Float = 16.0/9.0;
pub const IMAGE_WIDTH:i32 = 640;
pub const IMAGE_HEIGHT:i32 = (IMAGE_WIDTH as Float / ASPECT_RATIO) as i32;
pub const BUFFER_SIZE:usize = (IMAGE_WIDTH * IMAGE_HEIGHT * 3) as usize;
pub const NS:i32 = 10;

fn main() {
	let mut scene = HittableList::new();

	let mat1 = Lambertian::new(vec3!(0.1,0.2,0.5));
	let mat2 = Metal::new(vec3!(0.8,0.6,0.2),0.1);
	let mat3 = Metal::new(vec3!(0.8,0.8,0.8),1.0);
	let mat4 = Lambertian::new(vec3!(0.8,0.8,0.0));
	let mat5 = Dielectric::new(1.5);
	let mat6 = Dielectric::new(1.5);
	scene.add(Sphere::new(vec3!(0.0,0.0,-1.0),0.5,Arc::new(mat1)));
	scene.add(Sphere::new(vec3!(1.0,0.0,-1.0),0.5,Arc::new(mat2)));
	scene.add(Sphere::new(vec3!(-1.0,0.0,-1.0),0.5,Arc::new(mat5)));
	scene.add(Sphere::new(vec3!(-1.0,0.0,-1.0),-0.45,Arc::new(mat6)));
	scene.add(Sphere::new(vec3!(0.0,-100.5,-1.0),100.0,Arc::new(mat4)));

	let scene = scene;

	let camera = Camera::new(120.0,ASPECT_RATIO,1.0);

	let mut frame = Frame::new(IMAGE_WIDTH, IMAGE_HEIGHT);

	println!("Begin ..");
	&frame.buffer.par_iter_mut().for_each(|pixel:&mut Pixel|{
		let x = pixel.x;
		let y = pixel.y;
		let mut col = vec3!(0);

		//平滑采样
		for _s in 0..NS {
			let u = (x as Float + drand48()) / (IMAGE_WIDTH - 1) as Float;
			let v = (y as Float + drand48()) / (IMAGE_HEIGHT -1) as Float;
			let ray = camera.get_ray(u,v);
			col = col + ray.color(&scene,0)
		}

		col = col / vec3!(NS);

		col = col.gamma2();

		pixel.color = col;
	});


	let buffer = frame.get_raw_buffer();

	println!("Done.");
	image::save_buffer("image.png", &buffer, IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32, ColorType::Rgb8).unwrap()
}
