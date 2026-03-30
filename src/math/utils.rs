//! Mathematical utility functions
//! 
//! This module contains various mathematical helper functions used throughout the game.

/// Clamp a value between a minimum and maximum
/// 
/// # Arguments
/// * `value` - The value to clamp
/// * `min` - The minimum value
/// * `max` - The maximum value
/// 
/// # Returns
/// The clamped value
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Linear interpolation between two values
/// 
/// # Arguments
/// * `a` - Start value
/// * `b` - End value
/// * `t` - Interpolation factor (0.0 to 1.0)
/// 
/// # Returns
/// Interpolated value
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Convert degrees to radians
/// 
/// # Arguments
/// * `degrees` - Angle in degrees
/// 
/// # Returns
/// Angle in radians
pub fn deg_to_rad(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

/// Convert radians to degrees
/// 
/// # Arguments
/// * `radians` - Angle in radians
/// 
/// # Returns
/// Angle in degrees
pub fn rad_to_deg(radians: f32) -> f32 {
    radians * 180.0 / std::f32::consts::PI
}

/// Check if a value is approximately equal to another within a tolerance
/// 
/// # Arguments
/// * `a` - First value
/// * `b` - Second value
/// * `tolerance` - Maximum allowed difference
/// 
/// # Returns
/// True if values are approximately equal
pub fn approx_eq(a: f32, b: f32, tolerance: f32) -> bool {
    (a - b).abs() <= tolerance
}

/// Get the sign of a number (-1, 0, or 1)
/// 
/// # Arguments
/// * `value` - The number to check
/// 
/// # Returns
/// -1 if negative, 0 if zero, 1 if positive
pub fn signum(value: f32) -> i32 {
    if value > 0.0 {
        1
    } else if value < 0.0 {
        -1
    } else {
        0
    }
}

/// Round a float to the nearest integer
/// 
/// # Arguments
/// * `value` - The value to round
/// 
/// # Returns
/// Rounded integer
pub fn round(value: f32) -> i32 {
    if value >= 0.0 {
        (value + 0.5) as i32
    } else {
        (value - 0.5) as i32
    }
}

/// Calculate the next power of two greater than or equal to the value
/// 
/// # Arguments
/// * `value` - The input value
/// 
/// # Returns
/// Next power of two
pub fn next_power_of_two(mut value: u32) -> u32 {
    if value == 0 {
        return 1;
    }
    
    value -= 1;
    value |= value >> 1;
    value |= value >> 2;
    value |= value >> 4;
    value |= value >> 8;
    value |= value >> 16;
    value + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(5, 0, 10), 5);
        assert_eq!(clamp(-5, 0, 10), 0);
        assert_eq!(clamp(15, 0, 10), 10);
    }

    #[test]
    fn test_lerp() {
        assert_eq!(lerp(0.0, 10.0, 0.5), 5.0);
        assert_eq!(lerp(0.0, 10.0, 0.0), 0.0);
        assert_eq!(lerp(0.0, 10.0, 1.0), 10.0);
    }

    #[test]
    fn test_deg_to_rad() {
        assert!(approx_eq(deg_to_rad(180.0), std::f32::consts::PI, 0.001));
        assert!(approx_eq(deg_to_rad(90.0), std::f32::consts::PI / 2.0, 0.001));
    }

    #[test]
    fn test_rad_to_deg() {
        assert!(approx_eq(rad_to_deg(std::f32::consts::PI), 180.0, 0.001));
        assert!(approx_eq(rad_to_deg(std::f32::consts::PI / 2.0), 90.0, 0.001));
    }

    #[test]
    fn test_approx_eq() {
        assert!(approx_eq(1.0, 1.001, 0.01));
        assert!(!approx_eq(1.0, 1.1, 0.01));
    }

    #[test]
    fn test_signum() {
        assert_eq!(signum(5.0), 1);
        assert_eq!(signum(-5.0), -1);
        assert_eq!(signum(0.0), 0);
    }

    #[test]
    fn test_round() {
        assert_eq!(round(3.2), 3);
        assert_eq!(round(3.8), 4);
        assert_eq!(round(-3.2), -3);
        assert_eq!(round(-3.8), -4);
    }

    #[test]
    fn test_next_power_of_two() {
        assert_eq!(next_power_of_two(0), 1);
        assert_eq!(next_power_of_two(1), 1);
        assert_eq!(next_power_of_two(5), 8);
        assert_eq!(next_power_of_two(16), 16);
        assert_eq!(next_power_of_two(17), 32);
    }
}
