/// Converts an RGB color to the CIE 1931 XYZ color space.
///
/// # Arguments
/// * `r` - The red component of the RGB color.
/// * `g` - The green component of the RGB color.
/// * `b` - The blue component of the RGB color.
///
/// # Returns
/// The XYZ color space representation of the RGB color. The tuple contains the X, Y, and Z components.
#[inline]
#[must_use]
pub fn from_rgb(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let f = |t: f32| -> f32 {
        if t <= 0.04045 {
            t / 12.92
        } else {
            ((t + 0.055) / 1.055).powf(2.4)
        }
    };

    let r = f(r as f32 / 255.0);
    let g = f(g as f32 / 255.0);
    let b = f(b as f32 / 255.0);

    (
        0.412391 * r + 0.357584 * g + 0.180481 * b, // X
        0.212639 * r + 0.715169 * g + 0.072192 * b, // Y
        0.019331 * r + 0.119195 * g + 0.950532 * b, // Z
    )
}

/// Converts the CIE 1931 XYZ color space to the RGB color space.
///
/// # Arguments
/// * `x` - The X component of the XYZ color.
/// * `y` - The Y component of the XYZ color.
/// * `z` - The Z component of the XYZ color.
///
/// # Returns
/// The RGB color space representation of the XYZ color. The tuple contains the red, green, and blue components.
#[inline]
#[must_use]
pub fn to_rgb(x: f32, y: f32, z: f32) -> (u8, u8, u8) {
    let f = |t: f32| -> f32 {
        if t > 0.0031308 {
            1.055 * t.powf(1.0 / 2.4) - 0.055
        } else {
            12.92 * t
        }
    };

    let r = f(3.240970 * x - 1.537383 * y - 0.498611 * z);
    let g = f(-0.969244 * x + 1.875968 * y + 0.041555 * z);
    let b = f(0.055630 * x - 0.203977 * y + 1.056972 * z);

    (
        (r * 255.0).round() as u8,
        (g * 255.0).round() as u8,
        (b * 255.0).round() as u8,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::black((0, 0, 0), (0.0, 0.0, 0.0))]
    #[case::white((255, 255, 255), (0.9505, 1.0000, 1.0886))]
    #[case::red((255, 0, 0), (0.4125, 0.2127, 0.0193))]
    #[case::green((0, 255, 0), (0.3576, 0.7152, 0.1192))]
    #[case::blue((0, 0, 255), (0.1804, 0.0722, 0.9502))]
    #[case::cyan((0, 255, 255), (0.53802, 0.7873, 1.0698))]
    #[case::magenta((255, 0, 255), (0.5928, 0.2848, 0.9699))]
    #[case::yellow((255, 255, 0), (0.7700, 0.9278, 0.1385))]
    fn test_from_rgb(#[case] rgb: (u8, u8, u8), #[case] xyz: (f32, f32, f32)) {
        // Act
        let (x, y, z) = from_rgb(rgb.0, rgb.1, rgb.2);

        // Assert
        assert!((x - xyz.0).abs() < 1e-3);
        assert!((y - xyz.1).abs() < 1e-3);
        assert!((z - xyz.2).abs() < 1e-3);
    }

    #[rstest]
    #[case::black((0.0, 0.0, 0.0), (0, 0, 0))]
    #[case::white((0.9505, 1.0000, 1.0886), (255, 255, 255))]
    #[case::red((0.4125, 0.2127, 0.0193), (255, 0, 0))]
    #[case::green((0.3576, 0.7152, 0.1192), (0, 255, 0))]
    #[case::blue((0.1804, 0.0722, 0.9502), (0, 0, 255))]
    #[case::cyan((0.53802, 0.7873, 1.0698), (0, 255, 255))]
    #[case::magenta((0.5928, 0.2848, 0.9699), (255, 0, 255))]
    #[case::yellow((0.7700, 0.9278, 0.1385), (255, 255, 0))]
    fn test_to_rgb(#[case] xyz: (f32, f32, f32), #[case] rgb: (u8, u8, u8)) {
        // Act
        let (r, g, b) = to_rgb(xyz.0, xyz.1, xyz.2);

        // Assert
        assert_eq!(r, rgb.0);
        assert_eq!(g, rgb.1);
        assert_eq!(b, rgb.2);
    }
}
