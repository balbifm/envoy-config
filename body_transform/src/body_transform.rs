use log::info;
use log::error;
use proxy_wasm::traits::{Context, HttpContext};
use proxy_wasm::types::{Action, LogLevel};
use quickxml_to_serde::{Config, xml_string_to_json};

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_http_context(|context_id, _root_context_id| -> Box<dyn HttpContext> {
        Box::new(HttpBodyTransform {
            context_id
        })
    });
}


struct HttpBodyTransform {
    context_id: u32,
}


impl Context for HttpBodyTransform {}

/// A new HttpContext gets created for each HTTP request. If we need to pass custom configuration
/// to the filter, then we need to implement RootContext also. RootContext gets created per each
/// worker thread per plugin.
impl HttpContext for HttpBodyTransform {
    // fn on_http_response_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
    //     // Clean the content-length as we are going to change the size
    //     self.set_http_response_header("content-length", None);
    //     Action::Continue
    // }

    fn on_http_response_body(&mut self, _body_size: usize, _end_of_stream: bool) -> Action {
        let response_size = self.get_property(vec!["response", "size"]).unwrap_or_default();
        // info!("Response size {}.", String::from_utf8_lossy(response_size.as_slice()));
        info!("Got {} , {}, HTTP Request body in #{}.", _body_size, _end_of_stream, self.context_id);
        if !_end_of_stream {
            // Wait -- we'll be called again when the complete body is buffered
            // at the host side.
            return Action::Pause;
        }
        // Since we returned "Pause" previuously, this will return the whole body.
        if let Some(body_bytes) = self.get_http_response_body(0, 40010) {
            let mut body_str = String::from_utf8(body_bytes).unwrap();
            body_str.push_str("-tail");
            // let json_result = xml_string_to_json(body_str, &Config::new_with_defaults());
            // let json_body = match json_result {
            //     Ok(the_body) => the_body,
            //     Err(error) => panic!("Problem opening the file: {:?}", error),
            // };
            //
            // let bytes = &json_body.to_string().into_bytes();
            // error!("{}", body_str);
            self.set_http_response_body(0, 0xffffffff, body_str.as_bytes());
        }
        Action::Continue
    }
}

