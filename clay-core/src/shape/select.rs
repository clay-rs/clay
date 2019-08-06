
use crate::shape::cube::Cube;

use crate::shape::sphere::Sphere;

#[macro_export]
macro_rules! shape_select {
    ($Select:ident, { $( $Enum:ident($Shape:ty) ),+ }) => {
        shape_select!($Select, { $( $Enum($Shape) )+ })
    };
    ($Select:ident, { $( $Enum:ident($Shape:ty), )+ }) => {
        $crate::entity_select!($Select, { $( $Enum($Shape) )+ });

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
                    Self::ocl_code(<$Shape>::ocl_shape_fn(), "__SHAPE", "false"),
                ].join("\n")
            }
            fn ocl_shape_fn() -> String {
                Self::ocl_fn("hit")
            }
        }
    };
}


shape_select!(MySelect, {
    Sphere(Sphere),
    Cube(Cube),
});
