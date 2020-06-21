use crate::types::Float;
use crate::material::{Material, Lambertian};
use std::fmt::{Display, Formatter, Error};
use std::sync::Arc;
use crate::vector::Vec3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub a:Vec3,//origin
    pub b:Vec3,//direction
}

impl Display for Ray {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(),Error> {
        f.write_str(format!("({}->{})",self.a,self.b).as_str())
    }
}

impl Ray {
    pub fn new()->Self{
        Ray{ a:Vec3::new(), b:Vec3::new()}
    }

    pub fn from(a:Vec3,b:Vec3)->Self{
        Ray{a,b}
    }

    pub fn origin(&self)->Vec3{
        self.a
    }

    pub fn direction(&self)->Vec3{
        self.b
    }

    pub fn at(&self,t:Float)->Vec3{
        self.a + self.b * vec3!(t)
    }

    pub fn color<T:Hittable>(&self, obj: &T, depth:i32) ->Vec3{
        let mut rec = HitRecord::new();
        if obj.hit(self,0.001,Float::MAX,&mut rec){
            let mut scattered = Ray::new();
            let mut attenuation = vec3!();
            if rec.material.scatter(self,&rec,&mut attenuation,&mut scattered) && depth < 1000{
                scattered.color(obj,depth+1)*attenuation
            }else {
                vec3!(0)
            }
        }else{
            let unit_direction = self.direction().make_unit_vector();
            let t = 0.5*(unit_direction.y()+1.0);
            vec3!(1) * vec3!(1.0-t) + vec3!(0.5,0.7,1.0) * vec3!(t)
        }
    }
}

pub struct HitRecord {
    pub t:Float,pub p:Vec3,pub normal:Vec3,pub material:Arc<dyn Material>
}

impl HitRecord {
    pub fn new()->Self{
        HitRecord{t:0.0,p:vec3!(0.0),normal:vec3!(0.0),material:Arc::new(Lambertian::new(vec3!(0.2)))}
    }
}

pub trait Hittable : Send+Sync {
    fn hit(&self,r:&Ray,t_min:Float,t_max:Float,rec:&mut HitRecord)->bool;
}