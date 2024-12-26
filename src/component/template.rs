use super::super::route::Route;
use super::nav::Nav;
use ::dioxus::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn Template() -> Element {
  rsx! {
    Nav { }
    Outlet::<Route> {}
  }
}
