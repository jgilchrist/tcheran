macro_rules! parameters {
    (
        $(
            ($param:ident, $size:expr, $type:ident, $name:expr)
        ),* $(,)?
    ) => {
        #[cfg(feature = "tuner")]
        #[derive(Clone)]
        pub struct Parameters {
            $(
                pub $param: [PhasedEval; $size],
            )*
        }

        #[cfg(feature = "tuner")]
        impl Parameters {
            pub fn new() -> Self {
                Self {
                    $(
                        $param: [PhasedEval::ZERO; $size],
                    )*
                }
            }

            #[expect(unused_assignments, reason = "The final idx value will never be used")]
            pub fn from_array(arr: &[crate::engine::eval::tuning::TunerEval; Trace::SIZE]) -> Self {
                let mut evals = [PhasedEval::ZERO; Trace::SIZE];

                for (i, param) in arr.iter().enumerate() {
                    evals[i] = param.to_phased_eval();
                }

                let mut parameter_components = Self::new();
                let mut idx = 0;

                $(
                    let param_len = parameter_components.$param.len();
                    parameter_components.$param.copy_from_slice(&evals[idx..idx + param_len]);
                    idx += param_len;
                )*

                parameter_components
            }
        }

        #[cfg(feature = "tuner")]
        impl std::fmt::Display for Parameters {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                $(
                    match stringify!($type) {
                        "array" => crate::engine::eval::tuning::print_array(f, &self.$param, $name)?,
                        "pst" => crate::engine::eval::tuning::print_pst(f, &self.$param, $name)?,
                        "single" => crate::engine::eval::tuning::print_single(f, &self.$param, $name)?,
                        _ => unimplemented!()
                    }
                )*

                Ok(())
            }
        }

        pub struct Trace {
            $(
                pub $param: [crate::engine::eval::tuning::TraceComponent; $size],
            )*
        }

        impl Trace {
            #[cfg(feature = "tuner")]
            pub const SIZE: usize = size_of::<Self>() / size_of::<crate::engine::eval::tuning::TraceComponent>();

            pub fn new() -> Self {
                Self {
                    $(
                        $param: [crate::engine::eval::tuning::TraceComponent::default(); $size],
                    )*
                }
            }

            #[cfg(feature = "tuner")]
            #[expect(unused_assignments, reason = "The final idx value will never be used")]
            #[expect(clippy::cast_precision_loss, reason = "known cast from i32 to f32")]
            pub fn non_zero_coefficients(&self) -> Vec<crate::engine::eval::tuning::NonZeroCoefficient> {
                let mut result = Vec::new();
                let mut idx = 0;

                $(
                    for (i, component) in self.$param.iter().enumerate() {
                        let coefficient = component.0;

                        if coefficient != 0 {
                            result
                                .push(crate::engine::eval::tuning::NonZeroCoefficient::new(idx + i, coefficient as f32));
                        }
                    }

                    idx += self.$param.len();
                )*

                result
            }
        }
    };
}
