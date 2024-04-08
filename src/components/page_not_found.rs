use ::dioxus::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
  rsx! {
    h1 {
      "Page Not Found"
    }
    pre {
      color: "red",
      "{route:?}"
    }
  }
}
