mod canvas;
mod layer;
pub mod templates;

pub use {
  canvas::{
    Canvas,
    ImageFormat,
    to_rgba
  },
  layer::Layer
};
