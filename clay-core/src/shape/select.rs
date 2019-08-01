
#[macro_export]
macro_rules! shape_select {
    ($Select:ident, { $( $Enum:ident($Shape:ty) ),+ }) => {
        shape_select!($Select, { $( $Enum($Shape) )+ })
    };
    ($Select:ident, { $( $Enum:ident($Shape:ty), )+ }) => {
        pub enum $Select {
            $( $Enum($Shape), )+
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
            fn ocl_code() -> String {
                let mut cases = Vec::new();
                let mut i = 0;
                $(
                    cases.push([
                        format!("\tif (sel_idx == {}) {{", i),
                        format!("\t\treturn {}(__SHAPE_ARGS_B__(1, 0));", <$Shape>::ocl_shape_fn()),
                        "\t}".to_string(),
                    ].join("\n"));
                    i += 1;
                )+
                let cases_text = cases.join(" else\n");
                [
                    &format!("__SHAPE_RET__ {}(__SHAPE_ARGS_DEF__) {{", Self::ocl_fn()),
                    "\tint sel_idx = ibuf[0];",
                    &cases_text,
                    "\treturn false;",
                    "}",
                ].join("\n")
            }

            fn ocl_fn() -> String {
                use $crate::TypeHash;
                format!("__select_{:x}__", Self::type_hash())
            }
        }

        impl $crate::Shape for $Select {
            fn ocl_shape_code() -> String {
                [
                    $( <$Shape>::ocl_shape_code(), )+
                    Self::ocl_code(),
                ].join("\n")
            }
            fn ocl_shape_fn() -> String {
                Self::ocl_fn()
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
