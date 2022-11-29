// https://stackoverflow.com/questions/26388861/how-can-i-include-a-module-from-another-file-from-the-same-project

mod utils;
mod lvs;
mod pg;
mod qr;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    
    // Use `js_namespace` here to bind `console.log(..)` instead of just `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("vue-wasm alert!");
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

#[wasm_bindgen]
pub fn mult(a: i32, b: i32) -> i32 {
    return a * b;
}

// return r + &s;
// "111-222-333-444-555".to_string()
#[wasm_bindgen]
pub fn uuid() -> String {
    let r = String::from("hello rust");
    let c = String::from("console from rust");
    log(&c);
    return r;
}

#[wasm_bindgen]
pub fn levenshtein(a: &str, b: &str) -> i32 {    
    return lvs::levenshtein(&a, &b) as i32
}

#[wasm_bindgen]
pub fn generatepwd(
    nchars: i32, 
    specialchars: bool,
    uppercase: bool,
    lowercase: bool,
    numbers: bool) -> String {
    return pg::generatepwd(nchars,specialchars,uppercase,lowercase,numbers)
}

#[wasm_bindgen]
pub fn qrcode(arg: &str, width:i32, height:i32) -> String {
    let _width = width as u32;
    let _height = height as u32;
    return qr::qrcode_ffi(&arg, _width, _height)
}