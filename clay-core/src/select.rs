#[macro_export]
macro_rules! instance_select {
    ($Select:ident: $Base:path: $Class:ty { $( $Enum:ident($Param:ident = $Instance:ty) ),+ $(,)? }) => {
        pub enum $Select<
            $( $Param:
                $crate::Pack +
                $crate::Instance<$Class> +
                $Base
                = $Instance
            ),+
        > {
            $( $Enum($Param), )+
        }

        impl $Select {
            #[allow(unused_assignments)]
            fn index(&self) -> u32 {
                let mut i = 0;
                $(
                    if let $Select::$Enum(_) = self {
                        return i;
                    }
                    i += 1;
                )+
                unreachable!()
            }

            #[allow(unused_assignments)]
            fn method_source(method: &str) -> String {
                use $crate::class::*;
                let cpref = <$Class>::name().to_uppercase();

                let mut cases = Vec::new();
                let mut i = 0;
                $(
                    let inst_name = <$Instance as $crate::Instance<$Class>>::inst_name();
                    cases.push([
                        format!("\tif (sel_idx == {}) {{", i),
                        format!(
                            "\t\treturn {}_{}({}_ARGS_B(1, 0));",
                            inst_name, method, cpref,
                        ),
                        "\t}".to_string(),
                    ].join("\n"));
                    i += 1;
                )+
                let cases_text = cases.join(" else\n");
                [
                    &format!(
                        "{}_RET {}_{}({}_ARGS_DEF) {{",
                        cpref, Self::inst_name(), method, cpref,
                    ),
                    "\tint sel_idx = ibuf[0];",
                    &cases_text,
                    &format!("\treturn {}_RET_BAD;", cpref),
                    "}",
                ].join("\n")
            }
        }

        impl $crate::Instance<$Class> for $Select {
            fn source(cache: &mut std::collections::HashSet<u64>) -> String {
                use $crate::{TypeHash, class::*};
                if !cache.insert(Self::type_hash()) {
                    return String::new()
                }
                let mut ms = Vec::new();
                for method in <$Class>::methods().into_iter() {
                    ms.push(Self::method_source(&method));
                }
                [
                    $( <$Instance as $crate::Instance<$Class>>::source(cache), )+
                    ms.join("\n"),
                ].join("\n")
            }

            fn inst_name() -> String {
                use $crate::TypeHash;
                format!("__select_{:x}", Self::type_hash())
            }
        }

        impl $crate::Pack for $Select {
            fn size_int() -> usize {
                let sizes = [
                    $( <$Instance>::size_int(), )+
                ];
                1 + *sizes.iter().max().unwrap()
            }
            fn size_float() -> usize {
                let sizes = [
                    $( <$Instance>::size_float(), )+
                ];
                *sizes.iter().max().unwrap()
            }
            fn pack_to(&self, mut buffer_int: &mut [i32], buffer_float: &mut [f32]) {
                use $crate::pack::*;
                self.index().pack_int_to(buffer_int);
                buffer_int = &mut buffer_int[1..];
                match self {
                    $( $Select::$Enum(x) => x.pack_to(buffer_int, buffer_float), )+
                }
            }
        }

        $(
            impl From<$Instance> for $Select {
                fn from(origin: $Instance) -> Self {
                    $Select::$Enum(origin)
                }
            }
        )+
    };
}

#[allow(dead_code)]
mod _check {
    use crate::{
        shape::*,
        instance_select,
    };

    instance_select!(
        TestSelect: Shape: ShapeClass {
            Sphere(TS = UnitSphere),
            Cube(TC = UnitCube),
        }
    );
}
