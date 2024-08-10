use serde::{Deserialize, Serialize};
use serde_json::to_string;

/// Represents a color using red, green, blue, and alpha (transparency) values.
#[derive(Copy, Clone, Deserialize, Serialize)]
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

    #[test]
    fn test_partialeq_color_true() {
        let a = Color::new(11, 22, 33, 44);
        let b = Color::new(11, 22, 33, 44);
        assert_eq!(a.eq(&b), true);
        assert_eq!(b.eq(&a), true);
    }

    #[test]
    fn test_partialeq_color_first_different_false() {
        let a = Color::new(11, 22, 33, 44);
        let b = Color::new(10, 22, 33, 44);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_color_second_different_false() {
        let a = Color::new(11, 22, 33, 44);
        let b = Color::new(11, 9, 33, 44);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_color_third_different_false() {
        let a = Color::new(11, 22, 33, 44);
        let b = Color::new(11, 22, 5, 44);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_color_fourth_different_false() {
        let a = Color::new(11, 22, 33, 44);
        let b = Color::new(11, 22, 33, 43);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_color_all_different_false() {
        let a = Color::new(11, 22, 33, 44);
        let b = Color::new(10, 201, 35, 43);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_to_json() {
        let input = Color::new(11, 22, 33, 44);
        let input_serialized = to_string(&input);
        assert_eq!(input_serialized.is_ok(), true);
        let input_serialized_string = input_serialized.ok().unwrap();
        assert_eq!(input_serialized_string, "{\"r\":11,\"g\":22,\"b\":33,\"a\":44}");
    }
}