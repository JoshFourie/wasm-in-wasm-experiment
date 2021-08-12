use wasm_bindgen::prelude::*;

extern crate wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C"
{
    fn get_counter() -> i32;

    fn add_to_counter(value: i32) -> i32;
}

#[wasm_bindgen]
pub fn app() -> i32
{
    add_to_counter(1);

    String::from("Noooooo");

    9
}
