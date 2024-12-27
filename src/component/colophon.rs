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

    p {
      "This open source repository for this website is hosted on GitHub:",
      br { },
      a {
        href: "https://github.com/david-wallace-croft/dioxus-demo",
        target: "_blank",
      "https://github.com/david-wallace-croft/dioxus-demo",
      }
    }

    HighFive { }
  }
}
