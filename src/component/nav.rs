use super::super::route::Route;
use ::dioxus::prelude::*;

#[component]
pub fn Nav() -> Element {
  rsx! {
    nav {
      ul {
        li {
          Link {
            to: Route::Home {},
            "Home"
          }
        }
        li {
          Link {
            to: Route::Colophon {},
            "Colophon"
          }
        }
      }
    }
  }
}
