#[macro_export]
macro_rules! shape_select {
    ( $Select:ident { $( $Enum:ident ( $Param:ident = $Shape:ty ) ),+ $(,)? } ) => {
        $crate::instance_select!(
            $Select: $crate::Shape: $crate::ShapeClass {
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
        > $crate::Bounded<B_> for $Select<
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

#[cfg(test)]
mod check {
    use crate::{
        shape::test::TestShape,
        shape_select,
    };

    shape_select!(
        TestShapeSelect {
            Shape1(T1 = TestShape<i32>),
            Shape2(T2 = TestShape<f32>),
        }
    );
}
