use super::super::route::Route;
use super::nav::Nav;
use ::dioxus::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn Template() -> Element {
  const FAVICON: Asset = asset!("/assets/favicon.ico");

  const STYLESHEET: Asset = asset!("/assets/stylesheet.css");

  rsx! {
    document::Link {
      href: FAVICON,
      rel: "icon",
    }
    document::Link {
      href: STYLESHEET,
      rel: "stylesheet",
    }
    Nav { }
    Outlet::<Route> {}
  }
}
