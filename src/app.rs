#[macro_use]
mod macros;
mod environment;
mod router;
mod pages {
    export!(home);
    export!(about);
    export!(blog);
}

use router::router;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn app() {
    router();
}
