use std::{any::Any, ffi::OsStr};

use libloading::{Library, Symbol};
use tracing::{debug, log::trace};

use crate::model::{RawPayload, JsonPayload};

/// Maintain internal context of a `Plugin`.
/// This is passed to plugin in each handling function call.
#[derive(Debug, Clone, Default)]
pub struct PluginContext {}

/// Plugin interface to be implemented by the plugin author.
pub trait Plugin: Any + Send + Sync {

    /// Get identifier of the `Plugin` used to distinguish different plugins
    /// from each other.
    fn identifier(&self) -> &'static str;

    /// Get name describing the `Plugin`.
    fn name(&self) -> &'static str;

    /// A callback fired immediately after the plugin is loaded.
    /// This is usually used for initializing.
    fn on_plugin_load(&self) {}

    /// A callback fired immediately before the plugin is unloaded.
    /// This is most fit to be used for doing any cleanup.
    fn on_plugin_unload(&self) {}

    /// Handle payload receiving. The outcome of the handling should be mutated to the passed in `PluginContext`.
    fn raw_payload_recv(&self, _ctx: &mut PluginContext, _payload: &RawPayload) {}

    /// Handle payload receiving. The outcome of the handling should be mutated to the passed in `PluginContext`.
    fn json_payload_recv(&self, _ctx: &mut PluginContext, _payload: &JsonPayload) {}
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

/// Manage plugins and store their internal state.
pub struct PluginManager {
    /// Store each loaded library trait object as well as designated context object.
    plugins: Vec<(Box<dyn Plugin>, PluginContext)>,
    /// Let every library outlive its corresponding plugin.
    loaded_libraries: Vec<Library>,
}

impl PluginManager {
    pub fn new() -> PluginManager {
        PluginManager {
            plugins: Vec::new(),
            loaded_libraries: Vec::new(),
        }
    }

    /// Dynamically load a plugin from given file path.
    ///
    /// # Safety
    /// Each plugin to be loaded must contain a `_plugin_create` symbol
    /// in order to load. This symbol is exclusively defined in the plugin std library.
    pub unsafe fn load_path<P: AsRef<OsStr>>(&mut self, filename: P) -> anyhow::Result<()> {
        type PluginCreate = unsafe fn() -> *mut dyn Plugin;

        let lib = Library::new(filename.as_ref())?;

        self.loaded_libraries.push(lib);

        let lib = self.loaded_libraries.last().unwrap();

        let constructor: Symbol<PluginCreate> = lib.get(b"_plugin_create")?;

        let boxed_raw = constructor();

        let plugin = Box::from_raw(boxed_raw);
        debug!("Loaded plugin: {}", plugin.name());
        plugin.on_plugin_load();
        self.plugins.push((plugin, PluginContext::default()));

        Ok(())
    }

    /// Call raw payload hooks for each of the loaded plugins.
    pub fn raw_payload_recv(&mut self, payload: RawPayload) {
        debug!("Firing raw_payload_recv hooks");
        for (plugin, ctx) in &mut self.plugins {
            trace!("Firing raw_payload for {:?}", plugin.name());
            plugin.raw_payload_recv(ctx, &payload);
        }
    }

    /// Call json payload hooks for each of the loaded plugins.
    pub fn json_payload_recv(&mut self, payload: JsonPayload) {
        debug!("Firing json_payload_recv hooks");
        for (plugin, ctx) in &mut self.plugins {
            trace!("Firing json_payload for {:?}", plugin.name());
            plugin.json_payload_recv(ctx, &payload);
        }
    }
}
