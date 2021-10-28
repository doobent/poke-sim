

pub mod log {
    use wasm_bindgen::prelude::*;
    
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console)]
        pub fn log(s: &str);
    }

    macro_rules! console_log {
        ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
    }

    pub(crate) use console_log;
}

