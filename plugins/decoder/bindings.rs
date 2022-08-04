#[allow(clippy::all)]
mod decoder_plugin {
  #[derive(Clone)]
  pub struct Base64Packet {
    pub data: Vec<u8>,
  }
  impl core::fmt::Debug for Base64Packet {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      f.debug_struct("Base64Packet").field("data", &self.data).finish()}
  }
  #[derive(Clone)]
  pub struct JsonPacket {
    pub json: String,
  }
  impl core::fmt::Debug for JsonPacket {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      f.debug_struct("JsonPacket").field("json", &self.json).finish()}
  }
  #[repr(u8)]
  #[derive(Clone, Copy, PartialEq, Eq)]
  pub enum DecodingError {
    InvalidPacket,
  }
  impl core::fmt::Debug for DecodingError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      match self {
        DecodingError::InvalidPacket => {
          f.debug_tuple("DecodingError::InvalidPacket").finish()
        }
      }
    }
  }
  #[export_name = "identifier"]
  unsafe extern "C" fn __wit_bindgen_decoder_plugin_identifier() -> i32{
    let result = <super::DecoderPlugin as DecoderPlugin>::identifier();
    let ptr0 = __DECODER_PLUGIN_RET_AREA.0.as_mut_ptr() as i32;
    let vec1 = (result.into_bytes()).into_boxed_slice();
    let ptr1 = vec1.as_ptr() as i32;
    let len1 = vec1.len() as i32;
    core::mem::forget(vec1);
    *((ptr0 + 4) as *mut i32) = len1;
    *((ptr0 + 0) as *mut i32) = ptr1;
    ptr0
  }
  #[export_name = "name"]
  unsafe extern "C" fn __wit_bindgen_decoder_plugin_name() -> i32{
    let result = <super::DecoderPlugin as DecoderPlugin>::name();
    let ptr0 = __DECODER_PLUGIN_RET_AREA.0.as_mut_ptr() as i32;
    let vec1 = (result.into_bytes()).into_boxed_slice();
    let ptr1 = vec1.as_ptr() as i32;
    let len1 = vec1.len() as i32;
    core::mem::forget(vec1);
    *((ptr0 + 4) as *mut i32) = len1;
    *((ptr0 + 0) as *mut i32) = ptr1;
    ptr0
  }
  #[export_name = "on-plugin-load"]
  unsafe extern "C" fn __wit_bindgen_decoder_plugin_on_plugin_load(){
    let result = <super::DecoderPlugin as DecoderPlugin>::on_plugin_load();
    let () = result;
  }
  #[export_name = "on-plugin-unload"]
  unsafe extern "C" fn __wit_bindgen_decoder_plugin_on_plugin_unload(){
    let result = <super::DecoderPlugin as DecoderPlugin>::on_plugin_unload();
    let () = result;
  }
  #[export_name = "decode"]
  unsafe extern "C" fn __wit_bindgen_decoder_plugin_decode(arg0: i32, arg1: i32, ) -> i32{
    let len0 = arg1 as usize;
    let result = <super::DecoderPlugin as DecoderPlugin>::decode(Base64Packet{data:Vec::from_raw_parts(arg0 as *mut _, len0, len0), });
    let ptr1 = __DECODER_PLUGIN_RET_AREA.0.as_mut_ptr() as i32;
    match result {
      Ok(e) => { {
        *((ptr1 + 0) as *mut u8) = (0i32) as u8;
        let JsonPacket{ json:json2, } = e;
        let vec3 = (json2.into_bytes()).into_boxed_slice();
        let ptr3 = vec3.as_ptr() as i32;
        let len3 = vec3.len() as i32;
        core::mem::forget(vec3);
        *((ptr1 + 8) as *mut i32) = len3;
        *((ptr1 + 4) as *mut i32) = ptr3;
        
      } },
      Err(e) => { {
        *((ptr1 + 0) as *mut u8) = (1i32) as u8;
        *((ptr1 + 4) as *mut u8) = (match e {
          DecodingError::InvalidPacket => 0,
        }) as u8;
        
      } },
    };ptr1
  }
  
  #[repr(align(4))]
  struct __DecoderPluginRetArea([u8; 12]);
  static mut __DECODER_PLUGIN_RET_AREA: __DecoderPluginRetArea = __DecoderPluginRetArea([0; 12]);
  pub trait DecoderPlugin {
    fn identifier() -> String;
    fn name() -> String;
    fn on_plugin_load() -> ();
    fn on_plugin_unload() -> ();
    fn decode(source: Base64Packet,) -> Result<JsonPacket,DecodingError>;
  }
}
