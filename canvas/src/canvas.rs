use {
  crate::layer::Layer,
  ab_glyph::{
    Font,
    FontArc,
    PxScale,
    ScaleFont
  },
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
  lazy_static::lazy_static,
  regex::Regex,
  reqwest::Client,
  std::io::Cursor,
  unicode_segmentation::UnicodeSegmentation
};

lazy_static! {
  static ref DISCORD_EMOTE_REGEX: Regex = Regex::new(r"<a?:\w+:(\d+)>").expect("regex pattern failed");
}

#[derive(Default, Debug, Clone, Copy)]
pub enum ImageFormat {
  #[default]
  WebP,
  Jpeg {
    quality: u8
  }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum EmoteSource {
  Discord(String),
  Unicode(char)
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

/// Guess the width from text and size and font used then returns the assumed width
pub(crate) fn assume_text_width(
  text: &str,
  font_size: f32,
  font: &FontArc
) -> u32 {
  let scaled = font.as_scaled(PxScale::from(font_size));
  text.chars().map(|c| scaled.h_advance(scaled.glyph_id(c))).sum::<f32>().ceil() as u32
}

pub fn parse_all_emotes(s: &str) -> Vec<EmoteSource> {
  let mut out = Vec::new();
  let mut remaining = s;

  while !remaining.is_empty() {
    if let Some(mat) = DISCORD_EMOTE_REGEX.find(remaining) {
      for ch in remaining[..mat.start()].graphemes(true) {
        if ch.chars().next().unwrap().len_utf8() >= 3 {
          out.push(EmoteSource::Unicode(ch.chars().next().unwrap()));
          if out.len() == 3 {
            return out;
          }
        }
      }

      let caps = DISCORD_EMOTE_REGEX.captures(mat.as_str()).unwrap();
      let id = caps.get(1).unwrap().as_str().to_string();
      out.push(EmoteSource::Discord(id));
      if out.len() == 3 {
        return out;
      }

      remaining = &remaining[mat.end()..];
    } else {
      for ch in remaining.graphemes(true) {
        if ch.chars().next().unwrap().len_utf8() >= 3 {
          out.push(EmoteSource::Unicode(ch.chars().next().unwrap()));
          if out.len() == 3 {
            return out;
          }
        }
      }
      break;
    }
  }

  out
}

async fn reqwest_img(url: &str) -> Option<DynamicImage> {
  let http = Client::new();
  let resp = http.get(url).send().await.ok()?.bytes().await.ok()?;
  image::load_from_memory(&resp).ok()
}

pub(crate) async fn fetch_discord_emote(id: &str) -> Option<DynamicImage> {
  let url = format!("https://cdn.discordapp.com/emojis/{id}.webp?size=96");
  reqwest_img(&url).await
}

pub(crate) async fn fetch_twemoji_emote(c: char) -> Option<DynamicImage> {
  let cpt = format!("{:x}", c as u32);
  let version = "16.0.1";
  let url = format!("https://cdnjs.cloudflare.com/ajax/libs/twemoji/{version}/72x72/{cpt}.png");
  reqwest_img(&url).await
}
