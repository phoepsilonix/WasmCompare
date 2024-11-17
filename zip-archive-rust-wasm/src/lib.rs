use wasm_bindgen::prelude::*;
use js_sys::{Array, Uint8Array};
use std::io::Write;
use zip::{ZipWriter, write::FileOptions};
use std::io::Cursor;
use web_sys::console::log_1;

fn zip_error_to_js_value<E: std::fmt::Debug>(e: E) -> JsValue {
    JsValue::from_str(&format!("Zip error: {:?}", e))
}

#[wasm_bindgen]
pub fn archive(files: Array) -> Result<(), JsValue> {
    let mut zip_buffer = Cursor::new(Vec::new());
    let mut zip = ZipWriter::new(&mut zip_buffer);
    let options: FileOptions<'static, ()> = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .compression_level(Some(9));

    for i in 0..files.length() {
        let file = js_sys::Reflect::get(&files, &i.into())?;
        let name = js_sys::Reflect::get(&file, &"name".into())?
            .as_string()
            .ok_or_else(|| JsValue::from_str("Invalid file name"))?;
        let value = js_sys::Reflect::get(&file, &"value".into())?;
        let data = Uint8Array::new(&value).to_vec();

        zip.start_file(format!("zip_results/{}", &name), options).map_err(zip_error_to_js_value)?;
        zip.write_all(&data).map_err(zip_error_to_js_value)?;
    }

    zip.finish().map_err(zip_error_to_js_value)?;
    download( zip_buffer.into_inner(), "application/zip", "archive.zip");
    output_timestamp_log();
    Ok(())
}

fn download(zip_data: Vec<u8>, mime_type: &str, file_name: &str) {
    let u8_array = js_sys::Uint8Array::new_with_length(zip_data.len() as u32);
    u8_array.copy_from(&zip_data[..]);
    let blob_property_bag = web_sys::BlobPropertyBag::new();
    blob_property_bag.set_type(mime_type);

    let blob_parts = js_sys::Array::new();
    blob_parts.push(&JsValue::from(u8_array));

    let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(&blob_parts, &blob_property_bag).unwrap();

    // BlobからオブジェクトURLを作成
    let download_url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let a_elem = document.create_element("a").unwrap();
    let _ = a_elem.set_attribute("href", &download_url);
    let _ = a_elem.set_attribute("download", file_name);

    // ダウンロードリンクをクリックするイベントを発火
    let click_event = web_sys::MouseEvent::new("click").unwrap();
    a_elem.dispatch_event(&click_event).unwrap();
}

fn output_timestamp_log() {
    log_1(&JsValue::from(&format!("{}", js_sys::Date::now())));
}
