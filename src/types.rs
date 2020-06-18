use rand::Rng;
use crate::vector::Vec3;
use crate::material::Material;
use crate::ray::{Hittable, Ray, HitRecord};
use std::sync::Arc;

pub type Float = f64;

pub fn drand48()->Float{
	return rand::thread_rng().gen()
}

pub fn random_in_unit_sphere() ->Vec3{
	let mut p = vec3!();
	loop {
		p = vec3!(2) * vec3!(drand48(),drand48(),drand48()) - vec3!(1);
		if p.squared_length() < 1.0 {break;}
	}
	p
}

pub fn reflect(v:Vec3,n:Vec3)->Vec3{
	v-vec3!(v.dot(n)*2.0)*n
}

pub fn refract(v:Vec3,n:Vec3,ni_over_nt:Float,refracted:&mut Vec3)->bool{
	let uv = v.make_unit_vector();
	let dt = uv.dot(n);
	let discriminant = 1.0 - ni_over_nt*ni_over_nt*(1.0-dt*dt);
	if discriminant > 0.0{
		*refracted = vec3!(ni_over_nt)*(uv-n*vec3!(dt)) - n*vec3!(discriminant.sqrt());
		true
	}else {
		false
	}
}

pub fn schlick(cosine:Float,ref_idx:Float)->Float{
	let r0 = (1.0-ref_idx)/(1.0+ref_idx);
	let r0 = r0*r0;

	r0 + (1.0-r0)*(1.0-cosine).powf(5.0)
}

pub struct Sphere {
	pub center:Vec3,
	pub radius:Float,
	pub material:Arc<dyn Material>
}

impl Sphere {
	pub fn new(center:Vec3,radius:Float,material:Arc<dyn Material>)->Self {
		Sphere{center,radius,material}
	}
}

impl Hittable for Sphere {
	fn hit(&self,r: &Ray, t_min: Float, t_max: Float, rec:&mut HitRecord) -> bool {
		let oc = r.origin() - self.center;
		let a = r.direction().dot(r.direction());
		let b = oc.dot(r.direction());
		let c = oc.dot(oc) - self.radius*self.radius;
		let discriminant = b*b - a*c;
		if discriminant > 0.0 {
			let temp = (-b - (b*b-a*c).sqrt())/a;
			if temp < t_max && temp>t_min {
				rec.t = temp;
				rec.p = r.at(rec.t);
				rec.normal = (rec.p - self.center) / vec3!(self.radius);
				rec.material = self.material.clone();
				return true
			}
			let temp = (-b + (b*b-a*c).sqrt())/a;
			if temp < t_max && temp > t_min {
				rec.t = temp;
				rec.p = r.at(rec.t);
				rec.normal = (rec.p - self.center) / vec3!(self.radius);
				rec.material = self.material.clone();
				return true
			}
		}
		return false
	}
}

pub struct  HittableList {
	list:Vec<Box<dyn Hittable>>
}

impl HittableList {
	pub fn new()->Self{
		let list = Vec::with_capacity(1000);
		HittableList{list}
	}

	pub fn add<T:'static + Hittable>(&mut self,obj:T){
		let b = Box::new(obj);
		self.list.push(b)
	}

	pub fn list(&self)->&Vec<Box<dyn Hittable>>{
		&self.list
	}
}

impl Hittable for HittableList {
	fn hit(&self, r: &Ray, t_min: Float, t_max: Float, rec: &mut HitRecord) -> bool {
		let mut temp_rec = HitRecord::new();
		let mut hit_anything = false;
		let mut closest_so_far = t_max;

		for obj in &self.list {
			if obj.hit(r,t_min,closest_so_far,&mut temp_rec) {
				hit_anything = true;
				closest_so_far = temp_rec.t;

				rec.normal = temp_rec.normal;
				rec.p = temp_rec.p;
				rec.t = temp_rec.t;
				rec.material.clone_from(&temp_rec.material)
			}
		}

		hit_anything
	}
}

pub struct Camera {
	pub origin:Vec3,
	pub lower_left_corner:Vec3,
	pub horizontal:Vec3,
	pub vertical:Vec3,
	viewport:(Float,Float)
}

impl Camera {
	pub fn new(height:Float,focal_length:Float)->Self{
		let viewport = (height*(16.0/9.0),height);

		let horizontal = vec3!(viewport.0,0.0,0.0);
		let vertical = vec3!(0.0,viewport.1,0.0);
		let origin = vec3!(0);
		let lower_left_corner = origin - horizontal/vec3!(2) - vertical/vec3!(2) - vec3!(0,0,focal_length);

		println!("left corner {}",lower_left_corner);
		Camera{origin,lower_left_corner,horizontal,vertical,viewport}
	}

	pub fn get_ray(&self,u:Float,v:Float)->Ray{
		Ray::from(self.origin,self.lower_left_corner+self.horizontal*vec3!(u)+self.vertical*vec3!(v)-self.origin)
	}
}


#[derive(Copy, Clone)]
pub struct Pixel{
	pub x:i32,pub y:i32,
	pub color:Vec3
}

impl Pixel {
	pub fn new()->Self{
		Pixel{x:0,y:0,color:vec3!(0)}
	}

	pub fn r(&self)->u8{
		(self.color.r() * 255.999) as u8
	}

	pub fn g(&self)->u8{
		(self.color.g() * 255.999) as u8
	}

	pub fn b(&self)->u8{
		(self.color.b() * 255.999) as u8
	}
}

pub struct Frame{
	pub buffer:Vec<Pixel>,
	pub width:i32,pub height:i32,
}

impl Frame {
	pub fn new(width:i32,height:i32)->Self{
		let mut buffer = Vec::with_capacity((width * height) as usize);

		for j in 0..height {
			for i in 0..width {
				let mut pixel = Pixel::new();
				pixel.x = i;
				pixel.y = height - j - 1;
				buffer.push(pixel)
			}
		}

		Frame{width,height,buffer}
	}

	pub fn get_raw_buffer(&self)->Vec<u8>{
		let mut buffer= Vec::with_capacity((self.width * self.height * 3) as usize);
		for pixel in &self.buffer {
			buffer.push(pixel.r());
			buffer.push(pixel.g());
			buffer.push(pixel.b())
		}
		buffer
	}
}