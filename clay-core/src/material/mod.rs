

/// Material of an object surface.
/// It specifies the way how does ray bounce off the surface.
/// It defines the color, specularity, opacity, diffusion,
/// radiance and other properties of the object surface. 
pub trait Material {
    /// Associated OpenCL code that contains necessary function definition.
    fn ocl_emit_code() -> String;
    /// Name of the function from the code that is used to emit secondary rays.
    fn ocl_emit_fn() -> String;
}
