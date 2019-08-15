#[macro_export]
macro_rules! _replace {
    ($_t:tt, $sub:expr) => { $sub };
}

#[macro_export]
macro_rules! material_combine {
    ($Combine:ident { $( $field:ident : $Material:ty ),+ $(,)? }) => {
        pub struct $Combine {
            $( pub $field: (f64, $Material), )+
        }

        impl $Combine {
            pub fn new(
                $( mut $field: (f64, $Material), )+
            ) -> Self {
                let mut sum = 0.0;
                $(
                    sum += $field.0;
                    $field.0 = sum;
                )+
                Self {
                    $( $field: ($field.0*sum, $field.1), )+
                }
            }


            #[allow(unused_assignments)]
            fn method_source(method: &str) -> String {
                use $crate::{pack::*, class::*, material::*};

                let cpref = format!(
                    "{}_{}",
                    MaterialClass::name(),
                    method,
                ).to_uppercase();

                let mut cases = Vec::new();
                let (mut si, mut sf) = (0, 0);
                $(
                    let inst_name = <$Material as Instance<MaterialClass>>::inst_name();
                    cases.push([
                        format!("\tif (alpha < fbuf[{}]) {{", sf),
                        format!(
                            "\t\treturn {}_{}({}_ARGS_B({}, {}));",
                            inst_name, method, cpref, si, sf + 1,
                        ),
                        "\t}".to_string(),
                    ].join("\n"));
                    si += <$Material>::size_int();
                    sf += 1 + <$Material>::size_float();
                )+
                let cases_text = cases.join(" else\n");
                [
                    &format!(
                        "{}_RET {}_{}({}_ARGS_DEF) {{",
                        cpref, Self::inst_name(), method, cpref,
                    ),
                    "\tfloat alpha = random_uniform(seed);",
                    &cases_text,
                    &format!("\treturn {}_RET_BAD;", cpref),
                    "}",
                ].join("\n")
            }
        }

        impl $crate::Material for $Combine {
            fn brightness(&self) -> f64 {
                $(
                    self.$field.0*self.$field.1.brightness() +
                )+
                0.0
            }
        }

        impl $crate::Instance<$crate::material::MaterialClass> for $Combine {
            fn source(cache: &mut std::collections::HashSet<u64>) -> String {
                use $crate::{TypeHash, class::*, material::*};
                if !cache.insert(Self::type_hash()) {
                    return String::new()
                }
                let mut ms = Vec::new();
                for method in MaterialClass::methods().into_iter() {
                    ms.push(Self::method_source(&method));
                }
                [
                    $( <$Material as Instance<MaterialClass>>::source(cache), )+
                    ms.join("\n"),
                ].join("\n")
            }

            fn inst_name() -> String {
                use $crate::TypeHash;
                format!("__combine_{:x}", Self::type_hash())
            }
        }

        impl $crate::Pack for $Combine {
            fn size_int() -> usize {
                let sizes = [
                    $( <$Material>::size_int(), )+
                ];
                sizes.into_iter().sum::<usize>()
            }
            fn size_float() -> usize {
                let sizes = [
                    $( 1 + <$Material>::size_float(), )+
                ];
                sizes.into_iter().sum::<usize>()
            }
            fn pack_to(&self, buffer_int: &mut [i32], buffer_float: &mut [f32]) {
                use $crate::pack::*;
                Packer::new(buffer_int, buffer_float)
                $(
                    .pack(&self.$field.0)
                    .pack(&self.$field.1)
                )+;
            }
        }
    };
}

#[allow(dead_code)]
mod _check {
    use crate::{
        material::*,
        material_combine,
    };

    material_combine!(TestCombine {
        reflect: Reflective,
        diffuse: Diffuse,
    });
}
