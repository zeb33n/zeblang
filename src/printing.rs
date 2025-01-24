use wasm_bindgen::prelude::*;
#[wasm_bindgen(module = "/helpers.js")]
extern "C" {
    #[cfg(target_family = "wasm")]
    pub fn zeblang_print(s: &str);
}

#[cfg(not(target_family = "wasm"))]
pub fn zeblang_print(s: &str) {
    println!("{}", s);
}
