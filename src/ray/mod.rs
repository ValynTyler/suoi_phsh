use suoi_types::Vector3;


#[derive(Debug)]
pub enum Raycast {
    Miss,
    Hit(HitInfo),
}

impl Raycast {
    pub fn is_hit(&self) -> bool {
        match self {
            Raycast::Miss => false,
            Raycast::Hit(_) => true,
        }
    }
}

#[derive(Debug)]
pub struct HitInfo {
    pub distance: f32,
    pub point: Vector3,
    pub inside: bool,
}

pub struct Ray {
    point: Vector3,
    direction: Vector3,
}

impl Ray {
    pub fn point_dir(point: Vector3, direction: Vector3) -> Self {
        Self {
            point,
            direction,
        }
    }

    pub fn dir(&self) -> Vector3 {
        self.direction
    }

    pub fn pos(&self) -> Vector3 {
        self.point
    }
}
