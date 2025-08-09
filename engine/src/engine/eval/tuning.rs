use crate::chess::square::{File, Rank, Square};
use crate::engine::eval::PhasedEval;

#[derive(Clone)]
pub struct NonZeroCoefficient {
    pub idx: usize,
    pub value: f32,
}

impl NonZeroCoefficient {
    pub fn new(idx: usize, value: f32) -> Self {
        Self { idx, value }
    }
}

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

pub fn print_param(f: &mut std::fmt::Formatter<'_>, p: PhasedEval) -> std::fmt::Result {
    let (mg, eg) = (p.midgame().0, p.endgame().0);
    write!(f, "s({mg: >5}, {eg: >5})")
}

pub fn print_array(
    f: &mut std::fmt::Formatter<'_>,
    ps: &[PhasedEval],
    name: &str,
) -> std::fmt::Result {
    let size = ps.len();
    writeln!(f, "pub const {name}: [PhasedEval; {size}] = [")?;

    for param in ps {
        write!(f, "    ")?;
        print_param(f, *param)?;
        writeln!(f, ",")?;
    }

    writeln!(f, "];\n")?;

    Ok(())
}

pub fn print_pst(
    f: &mut std::fmt::Formatter<'_>,
    pst: &[PhasedEval],
    name: &str,
) -> std::fmt::Result {
    assert_eq!(pst.len(), Square::N);

    writeln!(f, "pub const {name}: PieceSquareTableDefinition = [")?;

    for rank in Rank::ALL.iter().rev() {
        write!(f, "    [")?;

        for file in File::ALL {
            let idx = Square::from_file_and_rank(file, *rank).array_idx();
            print_param(f, pst[idx])?;

            if file != File::H {
                write!(f, ", ")?;
            }
        }

        writeln!(f, "],")?;
    }

    writeln!(f, "];\n")?;

    Ok(())
}

pub fn print_single(
    f: &mut std::fmt::Formatter<'_>,
    p: &[PhasedEval],
    name: &str,
) -> std::fmt::Result {
    assert_eq!(p.len(), 1);

    write!(f, "pub const {name}: PhasedEval = ")?;
    print_param(f, p[0])?;
    writeln!(f, ";\n")?;

    Ok(())
}
