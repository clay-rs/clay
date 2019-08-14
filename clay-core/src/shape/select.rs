#[macro_export]
macro_rules! shape_select {
    ( $Select:ident { $( $Enum:ident ( $Param:ident = $Shape:ty ) ),+ $(,)? } ) => {
        $crate::instance_select!(
            $Select: $crate::shape::ShapeClass {
                $( $Enum($Param = $Shape) ),+
            }
        );
        impl Shape for $Select {}
        impl<
            B: Bound,
            $(
                $Param: 
                    $crate::Pack +
                    $crate::Instance<$crate::shape::ShapeClass> +
                    Bounded<B>
            ),+
        > Bounded<B> for $Select<
            $( $Param ),+
        > {
            fn bound(&self) -> B {
                
            }
        }
    };
}

mod check {
    use crate::{
        shape::*,
        shape_select,
    };

    shape_select!(
        TestSelect {
            Sphere(TS = UnitSphere),
            Cube(TC = UnitCube),
        }
    );
}
