pub static CORE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

/// Test declaration for investigating system functionality.
pub trait Function {
    fn call(&self, args: &[f64]) -> Result<f64, InvocationError>;

    fn help(&self) -> Option<&str> {
        None
    }
}

pub enum InvocationError {
    InvalidArgumentCount { expected: usize, found: usize },
    Other { msg: String },
}

pub trait PluginRegistrar {
    fn register_function(&mut self, name: &str, function: Box<dyn Function>);
}

pub struct PluginDeclaration {
    pub rustc_version: &'static str,
    pub core_version: &'static str,
    pub register: unsafe extern "C" fn(&mut dyn PluginRegistrar),
}

#[macro_export]
macro_rules! export_plugin {
    ($register:expr) => {
        #[doc(hidden)]
        #[no_mangle]
        pub static plugin_declaration: $crate::PluginDeclaration = $crate::PluginDeclaration {
            rustc_version: $crate::RUSTC_VERSION,
            core_version: $crate::CORE_VERSION,
            register: $register,
        };
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
