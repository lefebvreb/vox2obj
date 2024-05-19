#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl Vec3 {
    pub fn new<T: Into<u32>>(x: T, y: T, z: T) -> Self {
        Self { 
            x: x.into(), 
            y: y.into(), 
            z: z.into(),
        }
    }
}

#[derive(Debug)]
pub struct FaceVertex {
    pub v: usize,
    pub vt: usize,
    pub vn: usize,
}

impl FaceVertex {
    pub fn new(v: usize, vt: usize, vn: usize) -> Self {
        Self { v, vt, vn }
    }
}

pub type Face = [FaceVertex; 3];
