use super::super::route::Route;
use super::nav::Nav;
use ::dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Template() -> Element {
  rsx! {
    Nav { }
    Outlet::<Route> {}
  }
}
