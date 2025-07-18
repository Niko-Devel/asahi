mod canvas;
mod layer;
pub mod templates;
mod worker;

pub use {
  canvas::{
    Canvas,
    ImageFormat,
    parse_all_emotes,
    to_rgba
  },
  layer::{
    Font,
    Layer
  },
  worker::prefetch_emotes
};
