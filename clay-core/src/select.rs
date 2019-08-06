
#[macro_export]
macro_rules! entity_select {
    ($Select:ident, { $( $Enum:ident($Entity:ty) ),+ }) => {
        base_select!($Select, { $( $Enum($Shape) )+ })
    };
    ($Select:ident, { $( $Enum:ident($Entity:ty), )+ }) => {
        pub enum $Select {
            $( $Enum($Entity), )+
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
            fn ocl_code(ocl_fn: &str, ocl_cpref: &str, ocl_fn_ret: &str) -> String {
                let mut cases = Vec::new();
                let mut i = 0;
                $(
                    cases.push([
                        format!("\tif (sel_idx == {}) {{", i),
                        format!(
                            "\t\treturn {}({}_ARGS_B__(1, 0));",
                            ocl_fn, ocl_cpref,
                        ),
                        "\t}".to_string(),
                    ].join("\n"));
                    i += 1;
                )+
                let cases_text = cases.join(" else\n");
                [
                    &format!(
                        "{}_RET__ {}({}_ARGS_DEF__) {{",
                        ocl_cpref, ocl_fn, ocl_cpref,
                    ),
                    "\tint sel_idx = ibuf[0];",
                    &cases_text,
                    format!("\treturn {};", ocl_fn_ret),
                    "}",
                ].join("\n")
            }

            fn ocl_fn(ocl_fn_pref: &str) -> String {
                use $crate::TypeHash;
                format!("__select_{}_{:x}__", ocl_fn_pref, Self::type_hash())
            }
        }

        impl $crate::Pack for $Select {
            fn size_int() -> usize {
                let sizes = [
                    $( <$Shape>::size_int(), )+
                ];
                1 + *sizes.iter().max().unwrap()
            }
            fn size_float() -> usize {
                let sizes = [
                    $( <$Shape>::size_float(), )+
                ];
                *sizes.iter().max().unwrap()
            }
            fn pack_to(&self, mut buffer_int: &mut [i32], buffer_float: &mut [f32]) {
                use $crate::pack::*;
                self.index().pack_int_to(buffer_int);
                buffer_int = &mut buffer_int[1..];
                $(
                    if let $Select::$Enum(x) = self {
                        x.pack_to(buffer_int, buffer_float);
                        return;
                    }
                )+
                unreachable!()
            }
        }
    };
}
