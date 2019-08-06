#[macro_export]
macro_rules! _replace {
    ($_t:tt, $sub:expr) => { $sub };
}

#[macro_export]
macro_rules! material_combine {
    ($Combine:ident, { $( $field:ident : $Material:ty ),+ $(,)? }) => {
        pub struct $Combine {
            $( pub $field: ($Material, f64), )+
        }

        impl $Combine {
            pub fn new(
                $( mut $field: ($Material, f64), )+
            ) -> Self {
                let mut sum = 0.0;
                $(
                    sum += $field.1;
                    $field.1 = sum;
                )+
                Self {
                    $( $field: ($field.0, $field.1*sum), )+
                }
            }


            #[allow(unused_assignments)]
            fn method_source(method: &str) -> String {
                use $crate::{pack::*, class::*, material::*};

                let cpref = MaterialClass::name().to_uppercase();

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

        impl $crate::Material for $Combine {}

        impl $crate::Instance<$crate::material::MaterialClass> for $Combine {
            fn source() -> String {
                use $crate::{class::*, material::*};
                let mut ms = Vec::new();
                for method in MaterialClass::methods().into_iter() {
                    ms.push(Self::method_source(&method));
                }
                [
                    $( <$Material as Instance<MaterialClass>>::source(), )+
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
                    .pack(&self.$field.1)
                    .pack(&self.$field.0)
                )+;
            }
        }
    };
}
