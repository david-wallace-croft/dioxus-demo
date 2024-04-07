use ::dioxus::prelude::*;

fn main() {
  launch(App);
}

#[allow(non_snake_case)]
fn App() -> Element {
  rsx! {
      "Hello, World!"
  }
}
