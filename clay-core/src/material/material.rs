use crate::Pack;


/// Material of an object surface.
/// It specifies the way how does ray bounce off the surface.
/// It defines the color, specularity, opacity, diffusion,
/// radiance and other properties of the object surface. 
pub trait Material: Pack {
    /// Associated OpenCL code that contains necessary function definition.
    fn ocl_material_code() -> String;
    /// Name of the function from the code that is used to emit secondary rays.
    fn ocl_material_fn() -> String;
}
