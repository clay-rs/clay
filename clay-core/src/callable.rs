
#[macro_export]
macro_rules! declare_callable {
    ($class:expr, $methods:expr) => {
        declare_callable!($class, $methods,);
    };
    ($class:expr, $methods:expr,) => {
        /// Class name
        fn class() -> String {
            $class
        }
        /// List of methods of the class.
        fn methods() -> Vec<String> {
            $methods
        }
        /// Associated OpenCL code that contains necessary function definition.
        fn source() -> String;
        /// Name of the instance of the class.
        fn instance() -> String;
    };
}
