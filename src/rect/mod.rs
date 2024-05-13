use suoi_types::Vector2;

pub struct Rect {
    pub position: Vector2,
    pub size: Vector2,
}

impl Rect {
    #[rustfmt::skip]
    pub fn inside(&self, point: Vector2) -> bool {
        let position = self.position;
        let size = self.size;
        point.x < (position.x + size.x) &&
        point.x > (position.x - size.x) &&
        point.y < (position.y + size.y) &&
        point.y > (position.y - size.y)
    }
}
