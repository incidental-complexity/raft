#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct Color(u8, u8, u8);
impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color(red, green, blue)
    }

    pub fn unwrap(&self) -> (u8, u8, u8) {
        ((self.0).0, (self.1).0, (self.2).0)
    }
}

pub struct ColorStateMachine {
    color: Color,
}