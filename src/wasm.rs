use wasm_bindgen::prelude::*;

/// Console logging function for WebAssembly environment
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// Initialize the library for WebAssembly usage
#[wasm_bindgen(start)]
pub fn initialize() {
    // Set up panic hook for better error reporting in WASM
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

// Export all parsers for WebAssembly usage
pub use crate::parsers::wasm::AssignmentSubmitParser;
pub use crate::parsers::wasm::GradesParser;
pub use crate::parsers::wasm::PortalParser;
pub use crate::parsers::wasm::QuestionnaireParser;
pub use crate::parsers::wasm::StudentInfoInquiryParser;
pub use crate::parsers::wasm::SyllabusParser;
pub use crate::parsers::wasm::TestAnswerStatusParser;
