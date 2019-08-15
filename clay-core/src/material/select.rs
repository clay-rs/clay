#[macro_export]
macro_rules! material_select {
    ( $Select:ident { $( $Enum:ident ( $Param:ident = $Material:ty ) ),+ $(,)? } ) => {
        $crate::instance_select!(
            $Select: $crate::Material: $crate::material::MaterialClass {
                $( $Enum($Param = $Material) ),+
            }
        );
        impl Material for $Select {
            fn brightness(&self) -> f64 {
                match self {
                    $( $Select::$Enum(m) => m.brightness(), )+
                }
            }
        }
    };
}

#[allow(dead_code)]
mod _check {
    use crate::{
        material::*,
        material_select,
    };

    material_select!(
        TestSelect {
            Reflective(TR = Reflective),
            Diffuse(TD = Diffuse),
            Luminous(TL = Luminous),
        }
    );
}
