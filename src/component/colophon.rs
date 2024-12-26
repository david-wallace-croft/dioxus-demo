use super::high_five::HighFive;
use ::dioxus::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn Colophon() -> Element {
  rsx! {
    h1 { "Colophon Page" }
    p {
      "This website was created using the Rust library ",
      a {
        href: "https://dioxuslabs.com/",
        target: "_blank",
        "Dioxus",
      },
      "."
    }
    HighFive { }
  }
}
