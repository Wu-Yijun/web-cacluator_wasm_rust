pub trait SysFunctionReal {
    fn _get_epsilon_(&self) -> f64;

    // ----------------- inset functions -------------------- //
    fn _add(x: f64, y: f64) -> f64 {
        x + y
    }

    fn _sub(x: f64, y: f64) -> f64 {
        x - y
    }

    fn _mulitply(x: f64, y: f64) -> f64 {
        x * y
    }

    fn _devide(x: f64, y: f64) -> f64 {
        x / y
    }

    fn _abs(x: f64) -> f64 {
        x.abs()
    }

    fn _neg(x: f64) -> f64 {
        -x
    }

    fn _iszero(&self, x: f64) -> bool {
        let epsilon = self._get_epsilon_();
        x < epsilon && x > -epsilon
    }

    fn _round(x: f64) -> f64 {
        x.round()
    }

    fn _ceil(x: f64) -> f64 {
        x.ceil()
    }

    fn _floor(x: f64) -> f64 {
        x.floor()
    }

    fn _sin(x: f64) -> f64 {
        x.sin()
    }

    fn _cos(x: f64) -> f64 {
        x.cos()
    }

    fn _tan(x: f64) -> f64 {
        x.tan()
    }

    fn _arcsin(x: f64) -> f64 {
        x.asin()
    }

    fn _arccos(x: f64) -> f64 {
        x.acos()
    }

    fn _arctan(x: f64) -> f64 {
        x.atan()
    }

    /// * `x = 0`, `y = 0`: `0`
    /// * `x >= 0`: `arctan(y/x)` -> `[-pi/2, pi/2]`
    /// * `y >= 0`: `arctan(y/x) + pi` -> `(pi/2, pi]`
    /// * `y < 0`: `arctan(y/x) - pi` -> `(-pi, -pi/2)`
    fn _arctan2(x: f64, y: f64) -> f64 {
        x.atan2(y)
    }

    fn _cot(x: f64) -> f64 {
        let (s, c) = x.sin_cos();
        c / s
    }

    fn _sec(x: f64) -> f64 {
        1.0 / x.cos()
    }

    fn _csc(x: f64) -> f64 {
        1.0 / x.sin()
    }

    fn _arcsec(x: f64) -> f64 {
        (1.0 / x).acos()
    }

    fn _arccsc(x: f64) -> f64 {
        (1.0 / x).asin()
    }

    fn _arccot(x: f64) -> f64 {
        if x == 0.0 {
            std::f64::consts::FRAC_PI_2
        } else {
            (1.0 / x).atan()
        }
    }

    fn _sinh(x: f64) -> f64 {
        x.sinh()
    }

    fn _cosh(x: f64) -> f64 {
        x.cosh()
    }

    fn _tanh(x: f64) -> f64 {
        x.tanh()
    }

    fn _coth(x: f64) -> f64 {
        1.0 / x.tanh()
    }

    fn _sech(x: f64) -> f64 {
        1.0 / x.cosh()
    }

    fn _csch(x: f64) -> f64 {
        1.0 / x.sinh()
    }

    fn _arcsinh(x: f64) -> f64 {
        x.asinh()
    }

    fn _arccosh(x: f64) -> f64 {
        x.acosh()
    }

    fn _arctanh(x: f64) -> f64 {
        x.atanh()
    }

    fn _arccoth(x: f64) -> f64 {
        (1.0 / x).atanh()
    }

    fn _arcsech(x: f64) -> f64 {
        (1.0 / x).acosh()
    }

    fn _arccsch(x: f64) -> f64 {
        (1.0 / x).asinh()
    }

    fn _rad_to_deg(x: f64) -> f64 {
        x.to_degrees()
    }

    fn _deg_to_rad(x: f64) -> f64 {
        x.to_radians()
    }

    fn _square(x: f64) -> f64 {
        x * x
    }

    fn _sqrt(x: f64) -> f64 {
        x.sqrt()
    }

    fn _cube(x: f64) -> f64 {
        x * x * x
    }

    fn _cbrt(x: f64) -> f64 {
        x.cbrt()
    }

    /// x^y
    fn _pow(x: f64, y: f64) -> f64 {
        x.powf(y)
    }

    /// e^x
    fn _exp(x: f64) -> f64 {
        x.exp()
    }

    /// log_{x}(y)
    fn _log(x: f64, y: f64) -> f64 {
        y.log(x)
    }

    /// log_e(x)
    fn _ln(x: f64) -> f64 {
        x.ln()
    }

    /// log_10(x)
    fn _log10(x: f64) -> f64 {
        x.log10()
    }

    /// log_2(x)
    fn _log2(x: f64) -> f64 {
        x.log2()
    }
}
