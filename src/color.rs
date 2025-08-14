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

    pub fn write(self, scale: f64, f: &mut impl Write) -> std::io::Result<()> {
        let (r, g, b) = (self * scale).tuple();
        let rb = (256.0 * r.clamp(0.0, 0.999)).floor() as u8;
        let gb = (256.0 * g.clamp(0.0, 0.999)).floor() as u8;
        let bb = (256.0 * b.clamp(0.0, 0.999)).floor() as u8;
        write!(f, "{} {} {}\n", rb, gb, bb)
    }
}
