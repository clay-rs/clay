#[macro_export]
macro_rules! shape_select {
    ( $Select:ident { $( $Enum:ident ( $Param:ident = $Shape:ty ) ),+ $(,)? } ) => {
        $crate::instance_select!(
            $Select: $crate::Shape: $crate::shape::ShapeClass {
                $( $Enum($Param = $Shape) ),+
            }
        );
        impl $crate::Shape for $Select {}
        
        impl<
            B_: $crate::Bound,
            $(
                $Param: 
                    $crate::Shape +
                    $crate::Bounded<B_>
            ),+
        > Bounded<B_> for $Select<
            $( $Param ),+
        > {
            fn bound(&self) -> Option<B_> {
                match self {
                    $( $Select::$Enum(x) => x.bound(), )+
                }
            }
        }
    };
}

#[allow(dead_code)]
mod _check {
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
