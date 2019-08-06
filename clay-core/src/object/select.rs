#[macro_export]
macro_rules! object_select {
    ( $Select:ident, { $( $Enum:ident ( $Object:ty ) ),+ $(,)? } ) => {
        $crate::instance_select!(
            $Select:
            $crate::object::Object:
            $crate::object::ObjectClass,
            {
                $( $Enum($Object) ),+
            }
        );
    };
}
