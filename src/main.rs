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
pub const IMAGE_WIDTH:i32 = 1280;
pub const IMAGE_HEIGHT:i32 = (IMAGE_WIDTH as Float / ASPECT_RATIO) as i32;
pub const BUFFER_SIZE:usize = (IMAGE_WIDTH * IMAGE_HEIGHT * 3) as usize;
pub const NS:i32 = 100;

macro_rules! lambertian {
	($r:expr,$g:expr,$b:expr) => {
    	{
    		let r = $r as Float;
			let g = $g as Float;
			let b = $b as Float;
			Arc::new(Lambertian::new(Vec3::from(r,g,b)))
    	}
    };
}

macro_rules! metal {
	($r:expr,$g:expr,$b:expr,$fuzz:expr) => {
    	{
    		let r = $r as Float;
			let g = $g as Float;
			let b = $b as Float;
			let fuzz = $fuzz as Float;
			Arc::new(Metal::new(Vec3::from(r,g,b),$fuzz))
    	}
    };
}

macro_rules! dielectric {
	($ir:expr) => {
    	{
    		let ir = $ir as Float;
			Arc::new(Dielectric::new(ir))
    	}
    };
}

macro_rules! sphere {
	($center:expr,$r:expr,$mat:expr) => {
		{
			Sphere::new($center,$r,$mat)
		}
	}
}

fn main() {
	fn build_random_scene(scene:&mut HittableList){
		let earth = sphere!(vec3!(0,-1000,0),1000.0,lambertian!(0.5,0.5,0.5));
		scene.add(earth);

		for a in 0..22 {
			for b in 0..22 {
				let a = a-11;
				let b = b-11;
				let choose_mat = drand48();
				let center = vec3!(a as Float+0.9*drand48(),0.2,b as Float+0.9*drand48());
				if choose_mat < 0.8 {
					scene.add(sphere!(center,0.2,lambertian!(drand48()*drand48(),drand48()*drand48(),drand48()*drand48())))
				}else if choose_mat < 0.95 {
					scene.add(sphere!(center,0.2,metal!(0.5*(1.0+drand48()),0.5*(1.0+drand48()),0.5*(1.0+drand48()),0.5*drand48())))
				}else {
					scene.add(sphere!(center,0.2,dielectric!(1.5)))
				}
			}
		}

		scene.add(sphere!(vec3!(0,1,0),1.0,dielectric!(1.5)));
		scene.add(sphere!(vec3!(-4,1,0),1.0,lambertian!(0.4,0.2,0.1)));
		scene.add(sphere!(vec3!(4,1,0),1.0,metal!(0.7,0.6,0.5,0.0)));
	}

	let mut scene = HittableList::new();

	build_random_scene(&mut scene);

	let scene = scene;

	let lookfrom = vec3!(5,1,4);//vec3!(3,3,2);
	let lookat = vec3!(0,0,-1);//vec3!(0,0,-1);
	let focus_dist = (lookfrom-lookat).length();
	let camera = Camera::new(lookfrom,lookat,vec3!(0,1,0),45.0,ASPECT_RATIO,0.0,focus_dist);

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
	image::save_buffer("images/12-1.png", &buffer, IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32, ColorType::Rgb8).unwrap()
}