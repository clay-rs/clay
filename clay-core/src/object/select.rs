#[macro_export]
macro_rules! object_select {
    ( $Select:ident { $( $Enum:ident ( $Param:ident = $Object:ty ) ),+ $(,)? } ) => {
        $crate::instance_select!(
            $Select: $crate::Object: $crate::ObjectClass {
                $( $Enum($Param = $Object) ),+
            }
        );
        impl $crate::Object for $Select {}

        impl<
            B_: $crate::Bound,
            $(
                $Param: 
                    $crate::Object +
                    $crate::Bounded<B_>
            ),+
        > $crate::Bounded<B_> for $Select<
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
        > $crate::Targeted<T_> for $Select<
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

#[cfg(test)]
mod check {
    use crate::{
        shape::test::TestShape,
        material::test::TestMaterial,
        object::Covered,
        object_select,
    };

    object_select!(
        TestSelect {
            Object1(T1 = Covered<TestShape<i32>, TestMaterial<i32>>),
            Object2(T2 = Covered<TestShape<f32>, TestMaterial<f32>>),
        }
    );
}
