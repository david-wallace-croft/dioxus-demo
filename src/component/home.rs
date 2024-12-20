use super::high_five::HighFive;
use ::dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
  rsx! {
    h1 {
      "Home Page"
    }
    p {
      "This line is a placeholder for home page content."
    }
    HighFive { }
  }
}
