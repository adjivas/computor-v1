#[cfg(target_os="emscripten")]
extern crate webplatform;
extern crate computer_v1;

use computer_v1::prelude::Polynomial;

#[cfg(not(target_os="emscripten"))]
use std::env;
use std::str::FromStr;

#[cfg(target_os="emscripten")]
fn main() {
    let document = webplatform::init();
    let body = document.element_query("body").unwrap();
    body.html_set(r#"
        <h1>Computer V<sub>1</sub></h1>
        <form>
        <p><input id="text" name="text"></p>
        </form>
        <div>Type an expression</div>
    "#);

    let input = document.element_query("input").unwrap();
    let div = document.element_query("div").unwrap();
    input.on("input", move |event: webplatform::Event| {
        if let Some(input) = event.target {
            let program = input.prop_get_str("value");
            let result = Polynomial::from_str(&program).unwrap();

            div.html_set(&format!("{}", result));
        }
    });

    webplatform::spin();
}

#[cfg(not(target_os="emscripten"))]
fn main() {
    let merged: String =
        env::args().skip(1)
                   .filter_map(|arg: String| Polynomial::from_str(&arg).ok())
                   .map(|result: Polynomial| format!("{}", result))
                   .collect();
                
    print!("{}", merged);
}
