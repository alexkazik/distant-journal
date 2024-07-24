#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![allow(clippy::inline_always)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::similar_names)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unsafe_derive_deserialize)]

use crate::global::app::App;

pub(crate) mod data;
pub(crate) mod game;
pub(crate) mod global;
pub(crate) mod html;
pub(crate) mod pane;
pub(crate) mod route;
pub(crate) mod ser;

fn main() {
    yew::Renderer::<App>::new().render();
}
