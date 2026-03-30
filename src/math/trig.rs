//! Trigonometry lookup tables
//! 
//! This module implements fast trigonometry using pre-computed lookup tables,
//! equivalent to the `eo` class trigonometry optimization in the original Java code.

use std::sync::LazyLock;

/// Pre-computed sine lookup table with 65536 entries
/// 
/// This provides fast sine calculations without runtime computation,
/// matching the optimization strategy used in the original Minecraft.
static SIN_TABLE: LazyLock<[f32; 65536]> = LazyLock::new(|| {
    let mut table = [0.0f32; 65536];
    let mut i = 0;
    while i < 65536 {
        // Convert table index to angle in radians
        let angle = (i as f32) * 2.0 * std::f32::consts::PI / 65536.0;
        table[i] = libm::sinf(angle);
        i += 1;
    }
    table
});

/// Fast sine using lookup table
/// 
/// This is equivalent to `eo.a(float)` in the original Java code.
/// 
/// # Arguments
/// * `angle` - Angle in radians
/// 
/// # Returns
/// Sine of the angle
pub fn fast_sin(angle: f32) -> f32 {
    let index = (angle * 10430.378_f32) as i32;
    SIN_TABLE[(index as usize) & 0xFFFF]
}

/// Fast cosine using lookup table
/// 
/// This is equivalent to `eo.b(float)` in the original Java code.
/// Uses sine table with π/2 offset.
/// 
/// # Arguments
/// * `angle` - Angle in radians
/// 
/// # Returns
/// Cosine of the angle
pub fn fast_cos(angle: f32) -> f32 {
    let index = (angle * 10430.378_f32 + 16384.0_f32) as i32;
    SIN_TABLE[(index as usize) & 0xFFFF]
}

/// Fast square root
/// 
/// This is equivalent to `eo.c(float)` in the original Java code.
/// Uses libm for consistent results across platforms.
/// 
/// # Arguments
/// * `value` - Value to calculate square root for
/// 
/// # Returns
/// Square root of the value
pub fn fast_sqrt(value: f32) -> f32 {
    libm::sqrtf(value)
}

/// Floor function with proper negative handling
/// 
/// This is equivalent to `eo.d(float)` in the original Java code.
/// Handles negative numbers correctly unlike the default casting.
/// 
/// # Arguments
/// * `value` - Value to floor
/// 
/// # Returns
/// Largest integer less than or equal to value
pub fn fast_floor(value: f32) -> i32 {
    if value >= 0.0 {
        value as i32
    } else {
        let int_val = value as i32;
        if value == int_val as f32 {
            int_val
        } else {
            int_val - 1
        }
    }
}

/// Absolute value
/// 
/// This is equivalent to `eo.e(float)` in the original Java code.
/// 
/// # Arguments
/// * `value` - Value to get absolute value of
/// 
/// # Returns
/// Absolute value of the input
pub fn fast_abs(value: f32) -> f32 {
    if value < 0.0 { -value } else { value }
}

/// Maximum of two double values
/// 
/// This is equivalent to `eo.a(double, double)` in the original Java code.
/// 
/// # Arguments
/// * `a` - First value
/// * `b` - Second value
/// 
/// # Returns
/// Maximum of the two values
pub fn max_double(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}

/// Integer division with proper negative handling
/// 
/// This is equivalent to `eo.a(int, int)` in the original Java code.
/// Handles negative division correctly to match Java's behavior.
/// 
/// # Arguments
/// * `dividend` - Number to be divided
/// * `divisor` - Number to divide by
/// 
/// # Returns
/// Result of integer division
pub fn safe_int_div(dividend: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        panic!("Division by zero");
    }
    
    // Match Java's integer division behavior for negative numbers
    let result = dividend / divisor;
    
    // Adjust if there's a remainder and signs are different
    if (dividend ^ divisor) < 0 && dividend % divisor != 0 {
        result - 1
    } else {
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn test_fast_sin_accuracy() {
        let test_angles = [0.0, PI/4.0, PI/2.0, PI, 3.0*PI/2.0, 2.0*PI];
        
        for &angle in &test_angles {
            let fast_result = fast_sin(angle);
            let libm_result = libm::sinf(angle);
            let diff = (fast_result - libm_result).abs();
            assert!(diff < 0.001, "Sin accuracy test failed for angle {}: diff = {}", angle, diff);
        }
    }

    #[test]
    fn test_fast_cos_accuracy() {
        let test_angles = [0.0, PI/4.0, PI/2.0, PI, 3.0*PI/2.0, 2.0*PI];
        
        for &angle in &test_angles {
            let fast_result = fast_cos(angle);
            let libm_result = libm::cosf(angle);
            let diff = (fast_result - libm_result).abs();
            assert!(diff < 0.001, "Cos accuracy test failed for angle {}: diff = {}", angle, diff);
        }
    }

    #[test]
    fn test_fast_sqrt() {
        let test_values = [0.0, 1.0, 2.0, 4.0, 9.0, 16.0];
        
        for &value in &test_values {
            let fast_result = fast_sqrt(value);
            let libm_result = libm::sqrtf(value);
            let diff = (fast_result - libm_result).abs();
            assert!(diff < 0.001, "Sqrt accuracy test failed for value {}: diff = {}", value, diff);
        }
    }

    #[test]
    fn test_fast_floor() {
        assert_eq!(fast_floor(3.7), 3);
        assert_eq!(fast_floor(3.0), 3);
        assert_eq!(fast_floor(-3.7), -4);
        assert_eq!(fast_floor(-3.0), -3);
    }

    #[test]
    fn test_fast_abs() {
        assert_eq!(fast_abs(3.7), 3.7);
        assert_eq!(fast_abs(-3.7), 3.7);
        assert_eq!(fast_abs(0.0), 0.0);
    }

    #[test]
    fn test_max_double() {
        assert_eq!(max_double(3.7, 2.1), 3.7);
        assert_eq!(max_double(-1.5, -2.3), -1.5);
        assert_eq!(max_double(0.0, 0.0), 0.0);
    }

    #[test]
    fn test_safe_int_div() {
        assert_eq!(safe_int_div(7, 3), 2);
        assert_eq!(safe_int_div(-7, 3), -3);
        assert_eq!(safe_int_div(7, -3), -3);
        assert_eq!(safe_int_div(-7, -3), 2);
        assert_eq!(safe_int_div(6, 3), 2);
    }

    #[test]
    #[should_panic]
    fn test_safe_int_div_by_zero() {
        safe_int_div(5, 0);
    }
}
