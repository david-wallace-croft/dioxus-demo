use super::super::route::Route;
use super::nav::Nav;
use ::dioxus::prelude::*;

#[component]
pub fn Template() -> Element {
  rsx! {
    Nav { }
    Outlet::<Route> {}
  }
}
