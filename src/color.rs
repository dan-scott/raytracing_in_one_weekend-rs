pub(crate) use crate::vec3::Vec3 as Color;
use std::io::Write;

impl Color {
    pub fn r(&self) -> f64 {
        self.x()
    }
    pub fn g(&self) -> f64 {
        self.y()
    }
    pub fn b(&self) -> f64 {
        self.z()
    }

    pub fn write(self, f: &mut impl Write) -> std::io::Result<()> {
        let (r, g, b) = self.tuple();
        let rb = (255.999 * r).floor() as u8;
        let gb = (255.999 * g).floor() as u8;
        let bb = (255.999 * b).floor() as u8;
        write!(f, "{} {} {}\n", rb, gb, bb)
    }
}
