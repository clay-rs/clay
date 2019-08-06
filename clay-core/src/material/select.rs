#[macro_export]
macro_rules! material_select {
    ( $Select:ident, { $( $Enum:ident ( $Material:ty ) ),+ $(,)? } ) => {
        $crate::instance_select!(
            $Select:
            $crate::material::Material:
            $crate::material::MaterialClass,
            {
                $( $Enum($Material) ),+
            }
        );
    };
}
