use crate::engine::eval::PhasedEval;

// A non-packed version of PhasedEval
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct TunerEval(f32, f32);

pub const PHASE_COUNT_MAX: i16 = 24;

impl TunerEval {
    pub const ZERO: Self = Self(0.0, 0.0);

    pub const fn new(midgame: f32, endgame: f32) -> Self {
        Self(midgame, endgame)
    }

    pub const fn v(val: f32) -> Self {
        Self(val, val)
    }

    pub fn midgame(self) -> f32 {
        self.0
    }

    pub fn endgame(self) -> f32 {
        self.1
    }

    pub fn sqrt(self) -> Self {
        Self(self.0.sqrt(), self.1.sqrt())
    }

    #[expect(
        clippy::cast_possible_truncation,
        reason = "Intentionally truncating down to integers"
    )]
    pub fn to_phased_eval(self) -> PhasedEval {
        PhasedEval::new(self.0.round() as i16, self.1.round() as i16)
    }
}

impl std::ops::Add for TunerEval {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::AddAssign for TunerEval {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl std::ops::Sub for TunerEval {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::ops::SubAssign for TunerEval {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl std::ops::Mul for TunerEval {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl std::ops::Mul<f32> for TunerEval {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl std::ops::Div for TunerEval {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0, self.1 / rhs.1)
    }
}

impl std::iter::Sum for TunerEval {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut result = Self::ZERO;

        for i in iter {
            result += i;
        }

        result
    }
}
