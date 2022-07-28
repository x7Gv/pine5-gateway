use std::{any::Any, ffi::OsStr};

use libloading::{Library, Symbol};
use tracing::debug;

pub trait Plugin: Any + Send + Sync {
    fn name(&self) -> &'static str;

    fn on_plugin_road(&self) {}

    fn on_plugin_unload(&self) {}

    // fn on_payload_recv(&self) {}
}

macro_rules! declare_plugin {
    ($plugin_type:ty, $constructor:path) => {
        #[no_mangle]
        pub extern "C" fn _plugin_create() -> *mut $crate::Plugin {
            let constructor: fn() -> $plugin_type = $constructor;
            let object = constructor();
            let boxed: Box<$crate::Plugin> = Box::new(object);
            Box::into_raw(boxed)
        }
    };
}

pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
    loaded_libraries: Vec<Library>,
}

impl PluginManager {
    pub fn new() -> PluginManager {
        PluginManager {
            plugins: Vec::new(),
            loaded_libraries: Vec::new(),
        }
    }

    pub unsafe fn load_path<P: AsRef<OsStr>>(&mut self, filename: P) -> anyhow::Result<()> {
        type PluginCreate = unsafe fn() -> *mut dyn Plugin;

        let lib = Library::new(filename.as_ref())?;

        self.loaded_libraries.push(lib);

        let lib = self.loaded_libraries.last().unwrap();

        let constructor: Symbol<PluginCreate> = lib.get(b"_plugin_create")?;

        let boxed_raw = constructor();

        let plugin = Box::from_raw(boxed_raw);
        debug!("Loaded plugin: {}", plugin.name());
        plugin.on_plugin_road();
        self.plugins.push(plugin);

        Ok(())
    }
}
