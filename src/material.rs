use crate::types::{random_in_unit_sphere, reflect, Float, refract, schlick, drand48};
use crate::ray::{Ray, HitRecord};
use crate::vector::Vec3;

pub trait Material : Send+Sync {
    fn scatter(&self,ray_in:&Ray,rec:&HitRecord,attenuation:&mut Vec3,scattered:&mut Ray)->bool;
}

pub struct Lambertian {
    pub albedo:Vec3
}

impl Lambertian{
    pub fn new(albedo:Vec3)->Self{
        Lambertian{albedo}
    }
}

impl Material for Lambertian {
    //散射
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation:&mut Vec3, scattered: &mut Ray) -> bool {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        *scattered = Ray::from(rec.p,target - rec.p);
        *attenuation = self.albedo;
        return true
    }
}

pub struct Metal {
    pub albedo:Vec3,pub fuzz:Float
}

impl Metal {
    pub fn new(albedo:Vec3,fuzz:Float)->Self{
        let fuzz = if fuzz<1.0 { fuzz } else {1.0};
        Metal{albedo,fuzz}
    }
}

impl Material for Metal {
    //全反射
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = reflect(ray_in.direction().make_unit_vector(),rec.normal);
        *scattered = Ray::from(rec.p,reflected + vec3!(self.fuzz)*random_in_unit_sphere());
        *attenuation = self.albedo;
        // println!("attenuation {}",attenuation);
        return scattered.direction().dot(rec.normal)>0.0
    }
}

pub struct Dielectric{
    pub ref_idx:Float
}

impl Dielectric {
    pub fn new(ir:Float)->Self{
        Dielectric{ref_idx:ir}
    }
}

impl Material for Dielectric{

    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let mut outward_normal = vec3!(0);
        let reflected = reflect(ray_in.direction(),rec.normal);
        let mut ni_over_nt = 0.0;
        *attenuation = vec3!(1.0,1.0,1.0);
        let mut refracted = vec3!(0);
        let mut reflect_prod:Float = 0.0;
        let mut cosine:Float = 0.0;

        if ray_in.direction().dot(rec.normal) > 0.0 {
            outward_normal = rec.normal*vec3!(-1.0);
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * ray_in.direction().dot(rec.normal) / ray_in.direction().length();
        }else{
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -1.0 * (ray_in.direction().dot(rec.normal)) / ray_in.direction().length();
        }

        if refract(ray_in.direction(),outward_normal,ni_over_nt,&mut refracted){
            reflect_prod = schlick(cosine,self.ref_idx)
        }else{
            reflect_prod = 1.0;
        }

        if drand48() < reflect_prod {
            *scattered = Ray::from(rec.p,reflected)
        }else{
            *scattered = Ray::from(rec.p,refracted)
        }

        true
    }
}