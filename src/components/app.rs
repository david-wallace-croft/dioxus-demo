use super::super::route::Route;
use ::dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn App() -> Element {
  rsx! {
    Router::<Route> { }
  }
}
