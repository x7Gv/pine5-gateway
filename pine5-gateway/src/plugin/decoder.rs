use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    ffi::OsStr,
};

use wasmtime::*;
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};

use self::decoder_plugin::{DecoderPlugin, DecoderPluginData, JsonPacket};

wit_bindgen_wasmtime::import!("../interfaces/decoder_plugin.wit");

pub struct DecoderPluginInstance {
    pub plugin: DecoderPlugin<(WasiCtx, DecoderPluginData)>,
    pub store: Store<(WasiCtx, DecoderPluginData)>,
}

#[derive(Debug, Default)]
pub struct DecoderPluginContext {
    buffer: VecDeque<JsonPacket>,
}

pub struct DecoderPluginManager {
    pub plugins: HashMap<String, (DecoderPluginInstance, DecoderPluginContext)>,
    engine: Engine,
}

impl DecoderPluginManager {
    pub fn new() -> DecoderPluginManager {
        DecoderPluginManager {
            plugins: HashMap::new(),
            engine: Engine::default(),
        }
    }

    pub fn load_path<P: AsRef<OsStr>>(&mut self, file: P) -> anyhow::Result<()> {
        let engine = &self.engine;
        let module = Module::from_file(
            &engine,
            "../target/wasm32-wasi/release/pine5_decoder_plugin.wasm",
        )?;

        let mut linker = Linker::new(&engine);

        wasmtime_wasi::add_to_linker(&mut linker, |(wasi, _plugin_data)| wasi)?;

        let wasi = WasiCtxBuilder::new()
            .inherit_stdio()
            .inherit_args()?
            .build();

        let mut store = Store::new(&engine, (wasi, decoder_plugin::DecoderPluginData {}));

        let (plugin, _instance) = DecoderPlugin::instantiate(
            &mut store,
            &module,
            &mut linker,
            |(_wasi, plugin_data)| plugin_data,
        )?;

        plugin.on_plugin_load(&mut store)?;
        let identifier = plugin.identifier(&mut store)?;

        self.plugins.insert(
            identifier,
            (
                DecoderPluginInstance { plugin, store },
                DecoderPluginContext::default(),
            ),
        );

        Ok(())
    }

    pub fn decode(&mut self, packet: decoder_plugin::Base64Packet) -> anyhow::Result<()> {
        for (instance, ctx) in self.plugins.values_mut() {
            let plugin = &instance.plugin;
            let mut store = &mut instance.store;

            println!("Firing `decode()` for {}", plugin.name(&mut store)?);

            let packet = plugin.decode(store, packet.clone())?;
            packet
                .map(|packet| {
                    ctx.buffer.push_back(packet);
                })
                .unwrap();
        }

        Ok(())
    }
}
