use serde::Deserialize;
use std::io::{Cursor, Write};
use wasm_bindgen::prelude::*;
use web_sys::console::log_1;
use zip::write::FileOptions;

#[derive(Default, Deserialize)]
struct Data {
    name: String,
    #[serde(with="serde_bytes")]
    value: Vec<u8>,
}

#[wasm_bindgen]
pub fn archive(val: JsValue) {
    let data: Vec<Data> = serde_wasm_bindgen::from_value(val).unwrap();
    let mut bytes: Vec<u8> = Vec::new();
    {
    let mut cursor = Cursor::new(&mut bytes);
    let mut zip = zip::ZipWriter::new(&mut cursor);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .compression_level(Some(9));
 
    for v in data {
        zip.start_file(format!("zip_result/{}", v.name), options).unwrap();
        zip.write_all(&(v.value)[..]).unwrap();
    }
    zip.finish().unwrap();
    }

    download( bytes, "text/plain", "archive.zip");
}

fn download(result_bytes: Vec<u8>, mime_type: &str, file_name: &str) {
    let u8_array = js_sys::Uint8Array::new_with_length(result_bytes.len() as u32);
    u8_array.copy_from(&result_bytes[..]);
    let blob_parts = js_sys::Array::new();
    blob_parts.push(&u8_array.buffer());
    let blob_property_bag = web_sys::BlobPropertyBag::new();
    blob_property_bag.set_type(mime_type);
    let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(&blob_parts, &blob_property_bag).unwrap();
    let downlad_url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let a_elem = document.create_element("a").unwrap();
    let _ = a_elem.set_attribute("href", &downlad_url);
    let _ = a_elem.set_attribute("download", file_name);

    let mouse_event = web_sys::MouseEvent::new("click").unwrap();
    let _ = a_elem.dispatch_event(&web_sys::Event::from(mouse_event));
    output_timestamp_log();
}

fn output_timestamp_log() {
    log_1(&JsValue::from(&format!("{}", js_sys::Date::now())));
}
