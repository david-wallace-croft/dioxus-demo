use super::component::colophon::Colophon;
use super::component::home::Home;
use super::component::template::Template;
use ::dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
  #[layout(Template)]
  #[route("/")]
  Home {},
  #[route("/colophon")]
  Colophon {},
}
