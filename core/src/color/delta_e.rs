use crate::lab::Lab;
use crate::math::number::Float;
use crate::white_point::WhitePoint;

/// Enum representing the Delta E formula.
///
/// # Examples
/// ```
/// use auto_palette::delta_e::DeltaE;
/// use auto_palette::lab::Lab;
/// use auto_palette::white_point::D65;
///
/// let lab1 = Lab::<_, D65>::new(0.0, 0.0, 0.0);
/// let lab2 = Lab::<_, D65>::new(1.0, 1.0, 1.0);
/// let delta_e = DeltaE::CIE2000.measure(&lab1, &lab2);
/// assert!(delta_e > 0.0);
/// ```
///
/// # References
/// * [Delta E 101](http://zschuessler.github.io/DeltaE/learn/)
/// * [Color difference](https://en.wikipedia.org/wiki/Color_difference)
#[derive(Debug)]
pub enum DeltaE {
    /// The CIE76 formula.
    CIE76,
    /// The CIE94 formula.
    CIE94,
    /// The CIEDE2000 formula.
    CIE2000,
}

impl DeltaE {
    /// Measures the distance between two colors.
    ///
    /// # Type Parameters
    /// * `F` - The floating point type.
    /// * `WP` - The white point.
    ///
    /// # Arguments
    /// * `lab1` - The 1st color in CIE L*a*b* color space.
    /// * `lab2` - The 2nd color in CIE L*a*b* color space.
    ///
    /// # Returns
    /// The distance between the two colors.
    #[must_use]
    pub fn measure<F, WP>(&self, lab1: &Lab<F, WP>, lab2: &Lab<F, WP>) -> F
    where
        F: Float,
        WP: WhitePoint<F>,
    {
        match *self {
            DeltaE::CIE76 => cie76(lab1, lab2),
            DeltaE::CIE94 => cie94(
                lab1,
                lab2,
                F::from_f64(1.0),
                F::from_f64(0.045),
                F::from_f64(0.015),
            ),
            DeltaE::CIE2000 => ciede2000(lab1, lab2),
        }
    }
}

#[must_use]
fn cie76<F, WP>(lab1: &Lab<F, WP>, lab2: &Lab<F, WP>) -> F
where
    F: Float,
    WP: WhitePoint<F>,
{
    let delta_l = lab1.l - lab2.l;
    let delta_a = lab1.a - lab2.a;
    let delta_b = lab1.b - lab2.b;
    (delta_l.powi(2) + delta_a.powi(2) + delta_b.powi(2)).sqrt()
}

#[must_use]
fn cie94<F, WP>(lab1: &Lab<F, WP>, lab2: &Lab<F, WP>, k_l: F, k1: F, k2: F) -> F
where
    F: Float,
    WP: WhitePoint<F>,
{
    let delta_l = lab1.l - lab2.l;
    let delta_a = lab1.a - lab2.a;
    let delta_b = lab1.b - lab2.b;

    let c1 = (lab1.a.powi(2) + lab1.b.powi(2)).sqrt();
    let c2 = (lab2.a.powi(2) + lab2.b.powi(2)).sqrt();
    let delta_c = c1 - c2;
    let delta_h = (delta_a.powi(2) + delta_b.powi(2) - delta_c.powi(2)).sqrt();

    let s_l = F::one();
    let s_c = F::one() + k1 * c1;
    let s_h = F::one() + k2 * c1;
    (delta_l / (k_l * s_l)).powi(2) + (delta_c / s_c).powi(2) + (delta_h / s_h).powi(2)
}

#[must_use]
fn ciede2000<F, WP>(lab1: &Lab<F, WP>, lab2: &Lab<F, WP>) -> F
where
    F: Float,
    WP: WhitePoint<F>,
{
    let l_bar = (lab1.l + lab2.l) / F::from_f64(2.0);
    let delta_l_prime = lab2.l - lab1.l;

    let c1 = (lab1.a.powi(2) + lab1.b.powi(2)).sqrt();
    let c2 = (lab2.a.powi(2) + lab2.b.powi(2)).sqrt();
    let c_bar = (c1 + c2) / F::from_f64(2.0);

    let g = (c_bar.powi(7) / (c_bar.powi(7) + F::from_u32(25).powi(7))).sqrt();
    let a1_prime = lab1.a + (lab1.a / F::from_f64(2.0)) * (F::one() - g);
    let a2_prime = lab2.a + (lab2.a / F::from_f64(2.0)) * (F::one() - g);

    let c1_prime = (a1_prime.powi(2) + lab1.b.powi(2)).sqrt();
    let c2_prime = (a2_prime.powi(2) + lab2.b.powi(2)).sqrt();
    let c_bar_prime = (c1_prime + c2_prime) / F::from_f64(2.0);
    let delta_c_prime = c2_prime - c1_prime;

    let h_prime = |x: F, y: F| {
        if x.is_zero() && y.is_zero() {
            return F::zero();
        }

        let mut angle = y.atan2(x).to_degrees();
        if angle < F::zero() {
            angle += F::from_f64(360.0);
        }
        angle
    };

    let h1_prime = h_prime(a1_prime, lab1.b);
    let h2_prime = h_prime(a2_prime, lab2.b);

    let delta_h_prime = if c1_prime.is_zero() || c2_prime.is_zero() {
        F::zero()
    } else {
        let delta = h2_prime - h1_prime;
        if delta.abs() <= F::from_f64(180.0) {
            delta
        } else if h2_prime <= h1_prime {
            delta + F::from_f64(360.0)
        } else {
            delta - F::from_f64(360.0)
        }
    };
    #[allow(non_snake_case)]
    let delta_H_prime = F::from_f64(2.0)
        * (c1_prime * c2_prime).sqrt()
        * (delta_h_prime.to_radians() / F::from_f64(2.0)).sin();

    let h_bar_prime = if (h1_prime - h2_prime).abs() > F::from_f64(180.0) {
        (h1_prime + h2_prime + F::from_f64(360.0)) / F::from_f64(2.0)
    } else {
        (h1_prime + h2_prime) / F::from_f64(2.0)
    };

    let t = F::one() - F::from_f64(0.17) * (h_bar_prime - F::from_f64(30.0)).to_radians().cos()
        + F::from_f64(0.24) * (F::from_f64(2.0) * h_bar_prime).to_radians().cos()
        + F::from_f64(0.32)
            * (F::from_f64(3.0) * h_bar_prime + F::from_f64(6.0))
                .to_radians()
                .cos()
        - F::from_f64(0.20)
            * (F::from_f64(4.0) * h_bar_prime - F::from_f64(63.0))
                .to_radians()
                .cos();

    let s_l = F::one()
        + F::from_f64(0.015) * (l_bar - F::from_f64(50.0)).powi(2)
            / (F::from_f64(20.0) + (l_bar - F::from_f64(50.0)).powi(2)).sqrt();
    let s_c = F::one() + F::from_f64(0.045) * c_bar_prime;
    let s_h = F::one() + F::from_f64(0.015) * c_bar_prime * t;

    let r_t = F::from_f64(-2.0)
        * (c_bar_prime.powi(7) / (c_bar_prime.powi(7) + F::from_u32(25).powi(7))).sqrt()
        * (F::from_f64(60.0)
            * (-((h_bar_prime - F::from_f64(275.0)) / F::from_f64(25.0)).powi(2)).exp())
        .to_radians()
        .sin();

    let l = delta_l_prime / (F::from_f64(1.0) * s_l);
    let c = delta_c_prime / (F::from_f64(1.0) * s_c);
    let h = delta_H_prime / (F::from_f64(1.0) * s_h);
    (l * l + c * c + h * h + r_t * c * h).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lab::Lab;
    use crate::white_point::D65;
    use rstest::rstest;
    use statrs::assert_almost_eq;

    #[test]
    fn test_cie76() {
        let lab1 = Lab::<_, D65>::new(50.0, 2.6772, -79.7751);
        let lab2 = Lab::<_, D65>::new(50.0, 0.0, -82.7485);
        let actual = DeltaE::CIE76.measure(&lab1, &lab2);
        assert_almost_eq!(actual, 4.0010, 1e-4);
    }

    #[test]
    fn test_cie94() {
        let lab1 = Lab::<_, D65>::new(50.0, 2.6772, -79.7751);
        let lab2 = Lab::<_, D65>::new(50.0, 0.0, -82.7485);
        let actual = DeltaE::CIE94.measure(&lab1, &lab2);
        assert_almost_eq!(actual, 1.9461, 1e-4);
    }

    /// Test data from http://zschuessler.github.io/DeltaE/learn/
    #[rstest]
    #[case((0.0, 0.0, 0.0), (0.0, 0.0, 0.0), 0.0)]
    #[case((50.0, 2.6772, -79.7751), (50.0, 0.0, -82.7485), 2.0425)]
    #[case((50.0, 3.1571, -77.2803), (50.0, 0.0, -82.7485), 2.8615)]
    #[case((50.0, 2.8361, -74.0200), (50.0, 0.0, -82.7485), 3.4412)]
    #[case((50.0, -1.3802, -84.2814), (50.0, 0.0, -82.7485), 1.0)]
    #[case((50.0, -1.1848, -84.8006), (50.0, 0.0, -82.7485), 1.0)]
    #[case((50.0, -0.9009, -85.5211), (50.0, 0.0, -82.7485), 1.0)]
    #[case((50.0, 0.0, 0.0), (50.0, -1.0, 2.0), 2.3669)]
    #[case((50.0, -1.0, 2.0), (50.0, 0.0, 0.0), 2.3669)]
    #[case((50.0, 2.49, -0.001), (50.0, -2.49, 0.0009), 7.1792)]
    #[case((50.0, 2.49, -0.001), (50.0, -2.49, 0.0010), 7.1792)]
    #[case((50.0, 2.49, -0.001), (50.0, -2.49, 0.0011), 7.2195)]
    #[case((50.0, 2.49, -0.001), (50.0, -2.49, 0.0012), 7.2195)]
    #[case((50.0, -0.001, 2.49), (50.0, 0.0009, -2.49), 4.8045)]
    #[case((50.0, -0.001, 2.49), (50.0, 0.0010, -2.49), 4.8045)]
    #[case((50.0, -0.001, 2.49), (50.0, 0.0011, -2.49), 4.7461)]
    #[case((50.0, 2.5, 0.0), (50.0, 0.0, -2.5), 4.3065)]
    #[case((50.0, 2.5, 0.0), (73.0, 25.0, -18.0), 27.1492)]
    #[case((50.0, 2.5, 0.0), (61.0, -5.0, 29.0), 22.8977)]
    #[case((50.0, 2.5, 0.0), (58.0, 24.0, 15.0), 19.4535)]
    #[case((50.0, 2.5, 0.0), (50.0, 3.1736, 0.5854), 1.0)]
    #[case((50.0, 2.5, 0.0), (50.0, 3.2972, 0.0), 1.0)]
    #[case((50.0, 2.5, 0.0), (50.0, 1.8634, 0.5757), 1.0)]
    #[case((50.0, 2.5, 0.0), (50.0, 3.2592, 0.335), 1.0)]
    #[case((60.2574, -34.0099, 36.2677), (60.4626, -34.1751, 39.4387), 1.2644)]
    #[case((63.0109, -31.0961, -5.8663), (62.8187, -29.7946, -4.0864), 1.263)]
    #[case((61.2901, 3.7196, -5.3901), (61.4292, 2.248, -4.962), 1.8731)]
    #[case((35.0831, -44.1164, 3.7933), (35.0232, -40.0716, 1.5901), 1.8645)]
    #[case((22.7233, 20.0904, -46.694), (23.0331, 14.973, -42.5619), 2.0373)]
    #[case((36.4612, 47.858, 18.3852), (36.2715, 50.5065, 21.2231), 1.4146)]
    #[case((90.8027, -2.0831, 1.441), (91.1528, -1.6435, 0.0447), 1.4441)]
    #[case((90.9257, -0.5406, -0.9208), (88.6381, -0.8985, -0.7239), 1.5381)]
    #[case((6.7747, -0.2908, -2.4247), (5.8714, -0.0985, -2.2286), 0.6377)]
    #[case((2.0776, 0.0795, -1.135), (0.9033, -0.0636, -0.5514), 0.9082)]
    fn test_ciede2000(
        #[case] input1: (f64, f64, f64),
        #[case] input2: (f64, f64, f64),
        #[case] expected: f64,
    ) {
        let (l1, a1, b1) = input1;
        let (l2, a2, b2) = input2;
        let lab1 = Lab::<_, D65>::new(l1, a1, b1);
        let lab2 = Lab::<_, D65>::new(l2, a2, b2);
        let actual = DeltaE::CIE2000.measure(&lab1, &lab2);
        assert_almost_eq!(actual, expected, 1e-4);
    }
}
