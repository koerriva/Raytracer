use crate::types::Float;
use std::fmt::{Display, Error, Formatter};
use std::ops::{Add, Sub, Mul, Div};

#[derive(Copy, Clone)]
pub struct Vec3{
    e1: Float,e2: Float,e3: Float
}

impl Display for Vec3{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(),Error> {
        f.write_str(format!("[{},{},{}]",self.e1,self.e2,self.e3).as_str())
    }
}

impl Vec3 {
    pub fn new()->Self{
        Vec3{e1:0.0,e2:0.0,e3:0.0}
    }
    pub fn from(e1: Float, e2: Float, e3: Float) ->Self{
        Vec3{e1,e2,e3}
    }
    pub fn x(&self)-> Float {self.e1}
    pub fn y(&self)-> Float {self.e2}
    pub fn z(&self)-> Float {self.e3}

    pub fn r(&self)-> Float {self.e1}
    pub fn g(&self)-> Float {self.e2}
    pub fn b(&self)-> Float {self.e3}

    pub fn length(&self)-> Float {
        let e1 = self.e1;
        let e2 = self.e2;
        let e3 = self.e3;

        (e1*e1 + e2*e2 + e3*e3).sqrt()
    }

    pub fn squared_length(&self)-> Float {
        let e1 = self.e1;
        let e2 = self.e2;
        let e3 = self.e3;

        e1*e1 + e2*e2 + e3*e3
    }

    pub fn make_unit_vector(&self)-> Vec3 {
        let e1 = self.e1;
        let e2 = self.e2;
        let e3 = self.e3;

        let k:Float = 1.0/(e1*e1 + e2*e2 + e3*e3).sqrt();
        Vec3 {e1:e1*k,e2:e2*k,e3:e3*k}
    }

    pub fn dot(&self,v2:Vec3)->Float{
        self.e1*v2.e1 + self.e2*v2.e2 + self.e3*v2.e3
    }

    pub fn cross(&self,v2:Vec3)->Vec3{
        let e1 = self.e2*v2.e3 - self.e3*v2.e2;
        let e2 = -(self.e1*v2.e3 - self.e3*v2.e1);
        let e3 = self.e1*v2.e2 - self.e2*v2.e1;
        Vec3{e1,e2,e3}
    }

    pub fn unit_vector(&self,v:Vec3)->Vec3{
        v / Vec3::from(self.length(),self.length(),self.length())
    }

    pub fn gamma2(&self)->Vec3{
        let e1 = self.e1.sqrt();
        let e2 = self.e2.sqrt();
        let e3 = self.e3.sqrt();
        Vec3{e1,e2,e3}
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self,_rhs:Vec3)->Vec3{
        let e1 = self.e1 + _rhs.e1;
        let e2 = self.e2 + _rhs.e2;
        let e3 = self.e3 + _rhs.e3;
        Vec3 {e1,e2,e3}
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self,_rhs:Vec3)->Vec3{
        let e1 = self.e1 - _rhs.e1;
        let e2 = self.e2 - _rhs.e2;
        let e3 = self.e3 - _rhs.e3;
        Vec3 {e1,e2,e3}
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self,_rhs:Vec3)->Vec3{
        let e1 = self.e1 * _rhs.e1;
        let e2 = self.e2 * _rhs.e2;
        let e3 = self.e3 * _rhs.e3;
        Vec3 {e1,e2,e3}
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self,_rhs:Vec3)->Vec3{
        let e1 = self.e1 / _rhs.e1;
        let e2 = self.e2 / _rhs.e2;
        let e3 = self.e3 / _rhs.e3;
        Vec3 {e1,e2,e3}
    }
}

#[macro_export]
macro_rules! vec3 {
    ($e1:expr,$e2:expr,$e3:expr) => {
    	{
    		let e1 = $e1 as Float;
			let e2 = $e2 as Float;
			let e3 = $e3 as Float;
			Vec3::from(e1,e2,e3)
    	}
    };
    ($e:expr)=>{
    	{
    		let e = $e as Float;
    		Vec3::from(e,e,e)
    	}
    };
    ()=>{
    	Vec3::new()
    }
}

#[macro_export]
macro_rules! color {
    ($e1:expr,$e2:expr,$e3:expr) => {
    	vec3!($e1,$e2,$e3)
    };
}