use crate::color::white_points::WhitePoint;

/// Converts the CIE 1931 XYZ color space to the CIE L*a*b* color space.
///
/// # Type Parameters
/// * `WP` - The white point.
///
/// # Arguments
/// * `x` - The X component of the XYZ color.
/// * `y` - The Y component of the XYZ color.
/// * `z` - The Z component of the XYZ color.
///
/// # Returns
/// The L*a*b* color space representation of the XYZ color. The tuple contains the L, a, and b components.
#[inline]
#[must_use]
pub fn from_xyz<WP>(x: f32, y: f32, z: f32) -> (f32, f32, f32)
where
    WP: WhitePoint,
{
    let epsilon = (6.0 / 29.0_f32).powi(3);
    let kappa = 841.0 / 108.0; // ((29.0 / 6.0) ^ 2) / 3.0
    let delta = 4.0 / 29.0;

    let f = |t: f32| -> f32 {
        if t > epsilon {
            t.cbrt()
        } else {
            kappa * t + delta
        }
    };

    let fx = f(x / WP::x());
    let fy = f(y / WP::y());
    let fz = f(z / WP::z());

    (
        116.0 * fy - 16.0, // L
        500.0 * (fx - fy), // a
        200.0 * (fy - fz), // b
    )
}

/// Converts the CIE L*a*b* color space to the CIE 1931 XYZ color space.
///
/// # Type Parameters
/// * `WP` - The white point.
///
/// # Arguments
/// * `l` - The L component of the L*a*b* color.
/// * `a` - The a component of the L*a*b* color.
/// * `b` - The b component of the L*a*b* color.
///
/// # Returns
/// The XYZ color space representation of the L*a*b* color. The tuple contains the X, Y, and Z components.
#[inline]
#[must_use]
pub fn to_xyz<WP>(l: f32, a: f32, b: f32) -> (f32, f32, f32)
where
    WP: WhitePoint,
{
    let epsilon = 6.0 / 29.0;
    let kappa = 108.0 / 841.0; // 3.0 * ((6.0 / 29.0) ^ 2)
    let delta = 4.0 / 29.0;

    let f = |t: f32| -> f32 {
        if t > epsilon {
            t.powi(3)
        } else {
            kappa * (t - delta)
        }
    };

    let l2 = (l + 16.0) / 116.0;
    let fx = f(a / 500.0 + l2);
    let fy = f(l2);
    let fz = f(l2 - b / 200.0);

    (
        WP::x() * fx, // X
        WP::y() * fy, // Y
        WP::z() * fz, // Z
    )
}

#[cfg(test)]
mod tests {
    use crate::color::white_points::D65;
    use rstest::rstest;

    #[rstest]
    #[case::black((0.0, 0.0, 0.0), (0.0, 0.0, 0.0))]
    #[case::white((0.9505, 1.0000, 1.0886), (100.0, 0.0052, 0.0141))]
    #[case::red((0.4125, 0.2127, 0.0193), (53.2437, 80.09315, 67.2388))]
    #[case::green((0.3576, 0.7152, 0.1192), (87.7376, - 86.1846, 83.1813))]
    #[case::blue((0.1804, 0.0722, 0.9502), (32.3026, 79.1436, - 107.8436))]
    #[case::cyan((0.53802, 0.7873, 1.0698), (91.1120, - 48.0806, - 14.1521))]
    #[case::magenta((0.5928, 0.2848, 0.9699), (60.3199, 98.2302, - 60.8496))]
    #[case::yellow((0.7700, 0.9278, 0.1385), (97.1382, - 21.5551, 94.4825))]
    fn test_from_xyz(#[case] xyz: (f32, f32, f32), #[case] lab: (f32, f32, f32)) {
        // Act
        let (l, a, b) = super::from_xyz::<D65>(xyz.0, xyz.1, xyz.2);

        // Assert
        assert!((l - lab.0).abs() < 1e-3);
        assert!((a - lab.1).abs() < 1e-3);
        assert!((b - lab.2).abs() < 1e-3);
    }

    #[rstest]
    #[case::black((0.0, 0.0, 0.0), (0.0, 0.0, 0.0))]
    #[case::white((100.0, 0.0052, 0.0141), (0.9505, 1.0000, 1.0886))]
    #[case::red((53.2437, 80.09315, 67.2388), (0.4125, 0.2127, 0.0193))]
    #[case::green((87.7376, - 86.1846, 83.1813), (0.3576, 0.7152, 0.1192))]
    #[case::blue((32.3026, 79.1436, - 107.8436), (0.1804, 0.0722, 0.9502))]
    #[case::cyan((91.1120, - 48.0806, - 14.1521), (0.53802, 0.7873, 1.0698))]
    #[case::magenta((60.3199, 98.2302, - 60.8496), (0.5928, 0.2848, 0.9699))]
    #[case::yellow((97.1382, - 21.5551, 94.4825), (0.7700, 0.9278, 0.1385))]
    fn test_to_xyz(#[case] lab: (f32, f32, f32), #[case] xyz: (f32, f32, f32)) {
        // Act
        let (x, y, z) = super::to_xyz::<D65>(lab.0, lab.1, lab.2);

        // Assert
        assert!((x - xyz.0).abs() < 1e-3);
        assert!((y - xyz.1).abs() < 1e-3);
        assert!((z - xyz.2).abs() < 1e-3);
    }
}
