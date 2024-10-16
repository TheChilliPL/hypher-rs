use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = langExists)]
pub fn lang_exists(lang: &str) -> bool {
    parse_lang(lang).is_ok()
}

fn parse_lang(code: &str) -> Result<hypher::Lang, String> {
    if code.len() != 2 {
        return Err(format!("Expected a 2-character language code, got: {}", code));
    }

    let bytes: [u8; 2] = code.as_bytes().try_into().unwrap();

    hypher::Lang::from_iso(bytes).ok_or_else(|| format!("Invalid language code: {}", code))
}

#[wasm_bindgen]
pub fn hyphenate(word: &str, lang: &str) -> Result<Vec<String>, String> {
    let parsed_lang = parse_lang(lang).map_err(|e| e.to_string())?;
    
    Ok(hypher::hyphenate(word, parsed_lang).map(|s| s.to_string()).collect())
}
