mod counter;

use wasm_bindgen::prelude::wasm_bindgen;
use yew::App;

use counter::Counter;

#[wasm_bindgen(start)]
pub fn main() {
    App::<Counter>::new().mount_to_body();
}
