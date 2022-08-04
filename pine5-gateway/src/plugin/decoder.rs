use std::{ffi::OsStr, collections::{BTreeMap, HashMap}};

use wasmtime::*;
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};

use self::decoder_plugin::{DecoderPlugin, DecoderPluginData};

wit_bindgen_wasmtime::import!("../interfaces/decoder_plugin.wit");

pub struct DecoderPluginManager {
    pub plugins: HashMap<String, (DecoderPlugin<(WasiCtx, DecoderPluginData)>, Store<(WasiCtx, DecoderPluginData)>)>,
    engine: Engine,
}

impl DecoderPluginManager {
    pub fn new() -> DecoderPluginManager {
        DecoderPluginManager { plugins: HashMap::new(), engine: Engine::default() }
    }

    pub fn load_path<P: AsRef<OsStr>>(&mut self, file: P) -> anyhow::Result<()> {
        let engine = &self.engine;
        let module = Module::from_file(&engine, "../target/wasm32-wasi/release/pine5_decoder_plugin.wasm")?;

        let mut linker = Linker::new(&engine);

        wasmtime_wasi::add_to_linker(&mut linker, |(wasi, _plugin_data)| wasi)?;

        let wasi = WasiCtxBuilder::new()
            .inherit_stdio()
            .inherit_args()?
            .build();

        let mut store = Store::new(&engine, (wasi, decoder_plugin::DecoderPluginData {}));

        let (plugin, instance) =
            DecoderPlugin::instantiate(&mut store, &module, &mut linker, |(_wasi, plugin_data)| {
                plugin_data
            })?;

        plugin.on_plugin_load(&mut store)?;
        let identifier = plugin.identifier(&mut store)?;

        self.plugins.insert(identifier, (plugin, store));

        Ok(())
    }

    pub fn decode(&mut self) -> anyhow::Result<()> {

        for (plugin, store) in self.plugins.values_mut() {
            plugin.decode(store, decoder_plugin::Base64Packet { data: b"abc"} )?;
        }

        Ok(())
    }
}
