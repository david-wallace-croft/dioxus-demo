use ::dioxus::prelude::*;
use ::tracing::debug;

#[allow(non_snake_case)]
#[component]
pub fn HighFive() -> Element {
  let mut count: Signal<i32> = use_signal(|| 0);

  rsx! {
    h1 {
      "High-Five counter: {count}"
    }
    button {
      onclick: move |_mouse_event| {
        debug!("Increment");
  
        count += 1;
      },
      "Up high!"
    }
    button {
      onclick: move |_mouse_event| {
        debug!("Decrement");
  
        count -= 1;
      },
      "Down low!"
    }
  }
}
