mod components;

use self::components::app::App;

pub fn launch() {
  ::dioxus::prelude::launch(App);
}
