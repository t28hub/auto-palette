/// The D65 white point is the standard illuminant used in sRGB.
///
/// [Standard illuminant - Wikipedia](https://en.wikipedia.org/wiki/Standard_illuminant#Illuminant_series_D)
const D65_X: f32 = 0.950_470;
const D65_Y: f32 = 1.000_000;
const D65_Z: f32 = 1.088_830;

#[inline]
#[must_use]
pub fn from_xyz(x: f32, y: f32, z: f32) -> (f32, f32, f32) {
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

    let fx = f(x / D65_X);
    let fy = f(y / D65_Y);
    let fz = f(z / D65_Z);

    (
        116.0 * fy - 16.0, // L
        500.0 * (fx - fy), // a
        200.0 * (fy - fz), // b
    )
}

#[inline]
#[must_use]
pub fn to_xyz(l: f32, a: f32, b: f32) -> (f32, f32, f32) {
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
        D65_X * fx, // X
        D65_Y * fy, // Y
        D65_Z * fz, // Z
    )
}

#[cfg(test)]
mod tests {
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
        let (l, a, b) = super::from_xyz(xyz.0, xyz.1, xyz.2);

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
        let (x, y, z) = super::to_xyz(lab.0, lab.1, lab.2);

        // Assert
        assert!((x - xyz.0).abs() < 1e-3);
        assert!((y - xyz.1).abs() < 1e-3);
        assert!((z - xyz.2).abs() < 1e-3);
    }
}
