#[macro_export]
macro_rules! shape_select {
    ( $Select:ident, { $( $Enum:ident ( $Shape:ty ) ),+ $(,)? } ) => {
        $crate::instance_select!(
            $Select:
            $crate::shape::Shape:
            $crate::shape::ShapeClass,
            {
                $( $Enum($Shape) ),+
            }
        );
    };
}
