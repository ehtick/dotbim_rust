/// Represents a color using red, green, blue, and alpha (transparency) values.
#[derive(Copy, Clone)]
pub struct Color {
    /// The red component of the color.
    r: i32,
    /// The green component of the color.
    g: i32,
    /// The blue component of the color.
    b: i32,
    /// The alpha (transparency) component of the color.
    a: i32,
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
    }
}

impl Color {
    /// Returns a new Color
    pub fn new(r: i32, g: i32, b: i32, a: i32) -> Color { Color { r, g, b, a } }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_color() {
        let result = Color::new(11, 22, 33, 44);
        assert_eq!(result.r, 11);
        assert_eq!(result.g, 22);
        assert_eq!(result.b, 33);
        assert_eq!(result.a, 44);
    }
}