use {
  crate::layer::Layer,
  image::{
    DynamicImage,
    ExtendedColorType,
    ImageEncoder,
    ImageError,
    Rgba,
    RgbaImage,
    codecs::{
      jpeg::JpegEncoder,
      webp::WebPEncoder
    }
  },
  std::io::Cursor
};

#[derive(Default, Debug, Clone, Copy)]
pub enum ImageFormat {
  #[default]
  WebP,
  Jpeg {
    quality: u8
  }
}

pub struct Canvas {
  pub width:    u32,
  pub height:   u32,
  pub bg_color: Rgba<u8>,
  pub layers:   Vec<Layer>
}

impl Canvas {
  pub fn new(
    width: u32,
    height: u32
  ) -> Self {
    Self {
      width,
      height,
      bg_color: Rgba([0, 0, 0, 255]),
      layers: Vec::new()
    }
  }

  pub fn set_bg_color(
    &mut self,
    color: Rgba<u8>
  ) {
    self.bg_color = color;
  }

  pub fn add_layer(
    &mut self,
    layer: Layer
  ) {
    self.layers.push(layer);
  }

  /// Render the Canvas image
  pub fn render(&self) -> DynamicImage {
    let mut img = RgbaImage::from_pixel(self.width, self.height, self.bg_color);

    for layer in &self.layers {
      layer.render(&mut img);
    }

    DynamicImage::ImageRgba8(img)
  }

  /// Exports the image into bytes (Vec<u8>) with specified encoder
  pub fn to_bytes(
    &self,
    format: Option<ImageFormat>
  ) -> Result<Vec<u8>, ImageError> {
    let img = self.render();
    let rgba = img.to_rgba8();
    let mut buf = Vec::new();

    let format = format.unwrap_or_default();
    match format {
      ImageFormat::WebP => {
        let encoder = WebPEncoder::new_lossless(Cursor::new(&mut buf));
        encoder.write_image(&rgba, self.width, self.height, ExtendedColorType::Rgba8)?;
      },
      ImageFormat::Jpeg { quality } => {
        let encoder = JpegEncoder::new_with_quality(Cursor::new(&mut buf), quality);
        encoder.write_image(&img.to_rgb8(), self.width, self.height, ExtendedColorType::Rgb8)?;
      }
    }

    Ok(buf)
  }
}

/// Converts the value into [Rgba] format
pub fn to_rgba(color: u32) -> Rgba<u8> {
  let r = ((color >> 16) & 0xFF) as u8;
  let g = ((color >> 8) & 0xFF) as u8;
  let b = (color & 0xFF) as u8;
  Rgba([r, g, b, 255])
}
