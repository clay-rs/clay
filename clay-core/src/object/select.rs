#[macro_export]
macro_rules! object_select {
    ( $Select:ident { $( $Enum:ident ( $Param:ident = $Object:ty ) ),+ $(,)? } ) => {
        $crate::instance_select!(
            $Select: $crate::object::Object: $crate::object::ObjectClass {
                $( $Enum($Param = $Object) ),+
            }
        );
        impl Object for $Select {}

        impl<
            B_: $crate::Bound,
            $(
                $Param: 
                    $crate::Object +
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

        impl<
            T_: $crate::Target,
            $(
                $Param: 
                    $crate::Object +
                    $crate::Targeted<T_>
            ),+
        > Targeted<T_> for $Select<
            $( $Param ),+
        > {
            fn target(&self) -> Option<(T_, f64)> {
                match self {
                    $( $Select::$Enum(x) => x.target(), )+
                }
            }
        }
    };
}

#[allow(dead_code)]
mod _check {
    use crate::{
        shape::*,
        material::*,
        object::*,
        object_select,
    };

    object_select!(
        TestSelect {
            Sphere(TS = Covered<UnitSphere, Reflective>),
            Cube(TC = Covered<UnitCube, Reflective>),
        }
    );
}
