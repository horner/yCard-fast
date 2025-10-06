use crate::i18n::AliasManager;
use crate::schema::YCard;
use serde_json;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

// Global state for WASM using thread-safe alternatives
static ALIAS_MANAGER: OnceLock<Mutex<AliasManager>> = OnceLock::new();
static LAST_ERROR: OnceLock<Mutex<Option<String>>> = OnceLock::new();
static DOCUMENT_ARENA: OnceLock<Mutex<HashMap<i32, YCard>>> = OnceLock::new();
static NEXT_HANDLE: OnceLock<Mutex<i32>> = OnceLock::new();

fn get_alias_manager() -> &'static Mutex<AliasManager> {
    ALIAS_MANAGER.get_or_init(|| Mutex::new(AliasManager::new()))
}

fn get_document_arena() -> &'static Mutex<HashMap<i32, YCard>> {
    DOCUMENT_ARENA.get_or_init(|| Mutex::new(HashMap::new()))
}

fn get_next_handle() -> &'static Mutex<i32> {
    NEXT_HANDLE.get_or_init(|| Mutex::new(1))
}

fn set_last_error(error: &str) {
    let error_mutex = LAST_ERROR.get_or_init(|| Mutex::new(None));
    if let Ok(mut last_error) = error_mutex.lock() {
        *last_error = Some(error.to_string());
    }
}

fn get_last_error() -> Option<String> {
    let error_mutex = LAST_ERROR.get_or_init(|| Mutex::new(None));
    if let Ok(last_error) = error_mutex.lock() {
        last_error.clone()
    } else {
        None
    }
}

/// WASM exports for yCard core functionality
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn yc_parse(ptr: i32, len: i32) -> i32 {
    let slice = unsafe { std::slice::from_raw_parts(ptr as *const u8, len as usize) };
    let input = match std::str::from_utf8(slice) {
        Ok(s) => s,
        Err(e) => {
            set_last_error(&format!("Invalid UTF-8: {}", e));
            return -1;
        }
    };

    let alias_manager = if let Ok(manager) = get_alias_manager().lock() {
        manager.clone()
    } else {
        set_last_error("Failed to acquire alias manager lock");
        return -1;
    };

    let parser = crate::parser::Parser::with_alias_manager(alias_manager);
    match parser.parse_lenient(input, None) {
        Ok(ycard) => {
            let handle = if let Ok(mut next_handle) = get_next_handle().lock() {
                let current = *next_handle;
                *next_handle += 1;
                current
            } else {
                set_last_error("Failed to acquire handle lock");
                return -1;
            };

            if let Ok(mut arena) = get_document_arena().lock() {
                arena.insert(handle, ycard);
                handle
            } else {
                set_last_error("Failed to acquire arena lock");
                -1
            }
        }
        Err(e) => {
            set_last_error(&format!("Parse error: {}", e));
            -1
        }
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn yc_normalize(handle: i32, _flags: u32, locale_ptr: i32, locale_len: i32) -> i32 {
    let _locale = if locale_ptr != 0 && locale_len > 0 {
        let slice =
            unsafe { std::slice::from_raw_parts(locale_ptr as *const u8, locale_len as usize) };
        std::str::from_utf8(slice).ok()
    } else {
        None
    };

    if let Ok(arena) = get_document_arena().lock() {
        if arena.contains_key(&handle) {
            // Normalization would be applied here
            // For now, just return the same handle as normalization is done during parsing
            handle
        } else {
            set_last_error("Invalid handle");
            -1
        }
    } else {
        set_last_error("Failed to acquire arena lock");
        -1
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn yc_format(handle: i32, _flags: u32) -> i32 {
    if let Ok(arena) = get_document_arena().lock() {
        if let Some(ycard) = arena.get(&handle) {
            let formatter = crate::formatter::Formatter::new();
            match formatter.format(ycard) {
                Ok(formatted) => {
                    // Store formatted text and return pointer/length
                    // This is simplified - real implementation would manage memory properly
                    let bytes = formatted.into_bytes();
                    let ptr = bytes.as_ptr() as i32;
                    std::mem::forget(bytes); // Prevent deallocation
                    ptr
                }
                Err(e) => {
                    set_last_error(&format!("Format error: {}", e));
                    -1
                }
            }
        } else {
            set_last_error("Invalid handle");
            -1
        }
    } else {
        set_last_error("Failed to acquire arena lock");
        -1
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn yc_validate(handle: i32, mode: u32) -> i32 {
    let validation_mode = match mode {
        0 => crate::validator::ValidationMode::Lenient,
        1 => crate::validator::ValidationMode::Strict,
        _ => crate::validator::ValidationMode::Lenient,
    };

    if let Ok(arena) = get_document_arena().lock() {
        if let Some(ycard) = arena.get(&handle) {
            let validator = crate::validator::Validator::new(validation_mode);
            match validator.validate(ycard) {
                Ok(diagnostics) => match serde_json::to_string(&diagnostics) {
                    Ok(json) => {
                        let bytes = json.into_bytes();
                        let ptr = bytes.as_ptr() as i32;
                        std::mem::forget(bytes);
                        ptr
                    }
                    Err(e) => {
                        set_last_error(&format!("JSON serialization error: {}", e));
                        -1
                    }
                },
                Err(e) => {
                    set_last_error(&format!("Validation error: {}", e));
                    -1
                }
            }
        } else {
            set_last_error("Invalid handle");
            -1
        }
    } else {
        set_last_error("Failed to acquire arena lock");
        -1
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn yc_load_alias_pack(ptr: i32, len: i32) -> i32 {
    let slice = unsafe { std::slice::from_raw_parts(ptr as *const u8, len as usize) };
    let json = match std::str::from_utf8(slice) {
        Ok(s) => s,
        Err(e) => {
            set_last_error(&format!("Invalid UTF-8: {}", e));
            return -1;
        }
    };

    if let Ok(mut manager) = get_alias_manager().lock() {
        match manager.load_pack(json) {
            Ok(()) => 0,
            Err(e) => {
                set_last_error(&format!("Load pack error: {}", e));
                -1
            }
        }
    } else {
        set_last_error("Failed to acquire alias manager lock");
        -1
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn yc_clear_aliases() -> i32 {
    if let Ok(mut manager) = get_alias_manager().lock() {
        manager.clear_packs();
        0
    } else {
        set_last_error("Failed to acquire alias manager lock");
        -1
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn yc_set_default_locale(ptr: i32, len: i32) -> i32 {
    let slice = unsafe { std::slice::from_raw_parts(ptr as *const u8, len as usize) };
    let locale = match std::str::from_utf8(slice) {
        Ok(s) => s,
        Err(e) => {
            set_last_error(&format!("Invalid UTF-8: {}", e));
            return -1;
        }
    };

    if let Ok(mut manager) = get_alias_manager().lock() {
        manager.set_default_locale(locale);
        0
    } else {
        set_last_error("Failed to acquire alias manager lock");
        -1
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn yc_free(handle: i32) {
    if let Ok(mut arena) = get_document_arena().lock() {
        arena.remove(&handle);
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn yc_last_error() -> i32 {
    if let Some(error) = get_last_error() {
        let bytes = error.as_bytes();
        bytes.as_ptr() as i32
    } else {
        0
    }
}

/// High-level JavaScript API for LSP and other integrations
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn parse_ycard_lenient(input: &str, locale: Option<String>) -> Result<JsValue, JsValue> {
    use crate::parser::Parser;

    let parser = Parser::new();
    let locale_ref = locale.as_deref();
    match parser.parse_lenient(input, locale_ref) {
        Ok(ycard) => Ok(serde_wasm_bindgen::to_value(&ycard)?),
        Err(e) => Err(JsValue::from_str(&format!("Parse error: {}", e))),
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn parse_ycard_strict(input: &str) -> Result<JsValue, JsValue> {
    use crate::parser::Parser;

    let parser = Parser::new();
    match parser.parse_strict(input) {
        Ok(ycard) => Ok(serde_wasm_bindgen::to_value(&ycard)?),
        Err(e) => Err(JsValue::from_str(&format!("Parse error: {}", e))),
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn set_default_locale(locale: &str) -> Result<(), JsValue> {
    if let Ok(mut manager) = get_alias_manager().lock() {
        manager.set_default_locale(locale);
        Ok(())
    } else {
        Err(JsValue::from_str("Failed to acquire alias manager lock"))
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn load_alias_pack(content: &str) -> Result<(), JsValue> {
    if let Ok(mut manager) = get_alias_manager().lock() {
        match manager.load_pack_bytes(content.as_bytes()) {
            Ok(()) => Ok(()),
            Err(e) => Err(JsValue::from_str(&format!(
                "Failed to load alias pack: {}",
                e
            ))),
        }
    } else {
        Err(JsValue::from_str("Failed to acquire alias manager lock"))
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn validate_ycard(ycard_json: &str, mode: &str) -> Result<JsValue, JsValue> {
    use crate::validator::{ValidationMode, Validator};

    let validation_mode = match mode {
        "strict" => ValidationMode::Strict,
        "lenient" => ValidationMode::Lenient,
        _ => return Err(JsValue::from_str("Invalid validation mode")),
    };

    let ycard: YCard = serde_json::from_str(ycard_json)
        .map_err(|e| JsValue::from_str(&format!("Invalid yCard JSON: {}", e)))?;

    let validator = Validator::new(validation_mode);
    match validator.validate(&ycard) {
        Ok(diagnostics) => Ok(serde_wasm_bindgen::to_value(&diagnostics)?),
        Err(e) => Err(JsValue::from_str(&format!("Validation error: {}", e))),
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn format_ycard(ycard_json: &str, phones_style: &str) -> Result<String, JsValue> {
    use crate::formatter::{Formatter, PhonesStyle};

    let phones_style = match phones_style {
        "canonical" => PhonesStyle::Canonical,
        "shorthand" => PhonesStyle::Shorthand,
        "auto" => PhonesStyle::Auto,
        _ => return Err(JsValue::from_str("Invalid phones style")),
    };

    let ycard: YCard = serde_json::from_str(ycard_json)
        .map_err(|e| JsValue::from_str(&format!("Invalid yCard JSON: {}", e)))?;

    let formatter = Formatter::new().with_phones_style(phones_style);
    formatter
        .format(&ycard)
        .map_err(|e| JsValue::from_str(&format!("Format error: {}", e)))
}

// Compatibility wrapper functions for LSP server API
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn parse(input: &str, locale: Option<String>) -> Result<JsValue, JsValue> {
    parse_ycard_lenient(input, locale)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn validate(ycard_value: JsValue, _mode: JsValue) -> Result<JsValue, JsValue> {
    // Convert JsValue back to JSON string for validation
    let ycard_json = js_sys::JSON::stringify(&ycard_value)
        .map_err(|_| JsValue::from_str("Failed to stringify yCard"))?;
    let ycard_json_str = ycard_json
        .as_string()
        .ok_or_else(|| JsValue::from_str("Invalid JSON string"))?;

    // Use lenient validation by default
    validate_ycard(&ycard_json_str, "lenient")
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn format(ycard_value: JsValue, phones_style: JsValue) -> Result<String, JsValue> {
    // Convert JsValue back to JSON string for formatting
    let ycard_json = js_sys::JSON::stringify(&ycard_value)
        .map_err(|_| JsValue::from_str("Failed to stringify yCard"))?;
    let ycard_json_str = ycard_json
        .as_string()
        .ok_or_else(|| JsValue::from_str("Invalid JSON string"))?;

    // Extract phones style - default to canonical
    let style = if phones_style.is_string() {
        phones_style
            .as_string()
            .unwrap_or_else(|| "canonical".to_string())
    } else {
        "canonical".to_string()
    };

    format_ycard(&ycard_json_str, &style)
}

// Export aliases for compatibility with LSP server
#[cfg(feature = "wasm")]
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn setDefaultLocale(locale: &str) -> Result<(), JsValue> {
    set_default_locale(locale)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn loadAliasPack(content: &str) -> Result<(), JsValue> {
    load_alias_pack(content)
}

// C ABI exports (when not using WASM)
#[cfg(not(feature = "wasm"))]
pub mod c_api {
    use super::*;
    use std::ffi::{CStr, CString};
    use std::os::raw::{c_char, c_int};

    #[repr(C)]
    pub struct yc_error {
        pub code: c_int,
        pub message: *mut c_char,
    }

    #[repr(C)]
    pub struct yc_buffer {
        pub data: *mut u8,
        pub len: usize,
        pub capacity: usize,
    }

    #[no_mangle]
    pub extern "C" fn yc_load_alias_pack_bytes(
        bytes: *const u8,
        len: usize,
        out_err: *mut yc_error,
    ) -> c_int {
        if bytes.is_null() {
            return -1;
        }

        let slice = unsafe { std::slice::from_raw_parts(bytes, len) };
        if let Ok(mut manager) = get_alias_manager().lock() {
            match manager.load_pack_bytes(slice) {
                Ok(()) => 0,
                Err(e) => {
                    if !out_err.is_null() {
                        unsafe {
                            (*out_err).code = -1;
                            let msg = CString::new(format!("{}", e)).unwrap();
                            (*out_err).message = msg.into_raw();
                        }
                    }
                    -1
                }
            }
        } else {
            if !out_err.is_null() {
                unsafe {
                    (*out_err).code = -1;
                    let msg = CString::new("Failed to acquire alias manager lock").unwrap();
                    (*out_err).message = msg.into_raw();
                }
            }
            -1
        }
    }

    #[no_mangle]
    pub extern "C" fn yc_set_default_locale(
        locale_utf8: *const c_char,
        len: usize,
        out_err: *mut yc_error,
    ) -> c_int {
        if locale_utf8.is_null() {
            return -1;
        }

        let slice = unsafe { std::slice::from_raw_parts(locale_utf8 as *const u8, len) };
        match std::str::from_utf8(slice) {
            Ok(locale) => {
                if let Ok(mut manager) = get_alias_manager().lock() {
                    manager.set_default_locale(locale);
                    0
                } else {
                    if !out_err.is_null() {
                        unsafe {
                            (*out_err).code = -1;
                            let msg = CString::new("Failed to acquire alias manager lock").unwrap();
                            (*out_err).message = msg.into_raw();
                        }
                    }
                    -1
                }
            }
            Err(e) => {
                if !out_err.is_null() {
                    unsafe {
                        (*out_err).code = -1;
                        let msg = CString::new(format!("Invalid UTF-8: {}", e)).unwrap();
                        (*out_err).message = msg.into_raw();
                    }
                }
                -1
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn yc_clear_aliases(out_err: *mut yc_error) -> c_int {
        if let Ok(mut manager) = get_alias_manager().lock() {
            manager.clear_packs();
            0
        } else {
            if !out_err.is_null() {
                unsafe {
                    (*out_err).code = -1;
                    let msg = CString::new("Failed to acquire alias manager lock").unwrap();
                    (*out_err).message = msg.into_raw();
                }
            }
            -1
        }
    }
}
