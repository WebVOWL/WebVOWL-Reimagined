pub mod files {
    use wasm_bindgen_futures::JsFuture;
    use web_sys::js_sys::Uint8Array;

    /// Reads a JavaScript file handle and returns a Rust byte vector
    pub async fn read_web_file(web_file: web_sys::File) -> Vec<u8> {
        let prom_buf = web_file.array_buffer();
        let future = JsFuture::from(prom_buf);
        let buf = match future.await {
            Ok(res) => res,
            Err(error) => panic!("Failed to load file {error:?}"),
        };
        let array = Uint8Array::new(&buf.into());
        array.to_vec()
    }
}
