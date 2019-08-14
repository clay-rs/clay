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
        impl Material for $Select {
            fn brightness(&self) -> f64 {
                $(
                    if let $Select::$Enum(m) = self {
                        return m.brightness()
                    }
                )+
                unreachable!()
            }
        }
    };
}
