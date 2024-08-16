
extern crate nalgebra as na;

use core::fmt;
use std::fmt::Formatter;

// use fast_image_resize:: FilterType;
use na::Vector4;

#[derive(Copy, Clone)]
pub struct ResizeImage {
    pub width: u32,
    pub height: u32,
    // pub filter: FilterType
}
#[derive(Copy, Clone)]
pub struct UVVec {
    pub u:f32,
    pub v:f32,
    pub w:f32
}
impl UVVec {
    pub fn new(u:f32,v:f32,w:f32) -> UVVec {
        return UVVec {
            u,v,w
        }
    }
    pub fn default() -> UVVec {
        return UVVec::new(0.0,0.0,1.0)
    }
}
impl fmt::Debug for UVVec {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.u,self.v,self.w)
    }
} 

#[derive(Copy, Clone)]
pub struct UV {
    pub first: UVVec,
    pub second: UVVec,
    pub third: UVVec
}

impl fmt::Debug for UV {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {:?} {:?}", self.first,self.second,self.third,)
    }
} 

impl UV {
    pub fn new(first:UVVec,second:UVVec,third:UVVec) -> UV {
        return UV {
            first,second,third
        }
    }
    pub fn default() -> UV {
        return UV {
            first: UVVec::default(),
            second: UVVec::default(),
            third: UVVec::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct TriangleV4Plus {
    pub first: Vector4<f32>,
    pub second: Vector4<f32>,
    pub third: Vector4<f32>,
    pub dp:f32,
    pub uv: UV
}

impl TriangleV4Plus {
    pub fn new(first:Vector4<f32>,second:Vector4<f32>,third:Vector4<f32>,dp:f32,uv:UV) -> TriangleV4Plus {
        return TriangleV4Plus {
            first,second,third,dp,uv
        }
    }
    pub fn default() -> TriangleV4Plus {
        return TriangleV4Plus {
            first: Vector4::new(0.0,0.0,0.0,1.0),
            second: Vector4::new(0.0,0.0,0.0,1.0),
            third: Vector4::new(0.0,0.0,0.0,1.0),
            dp: 1.0,
            uv: UV::default()
        }
    }
}
impl fmt::Debug for TriangleV4Plus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} {} {:?}", self.first,self.second,self.third,self.dp,self.uv)
    }
} 

pub type MeshV4Plus = Vec<TriangleV4Plus>;


// type UV = Vector3<Vector2<f32>>;
