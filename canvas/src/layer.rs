use {
  ab_glyph::{
    FontArc,
    PxScale
  },
  image::{
    DynamicImage,
    GenericImageView,
    Rgba,
    RgbaImage,
    imageops::{
      Lanczos3,
      overlay
    }
  },
  imageproc::{
    drawing::{
      draw_filled_rect_mut,
      draw_line_segment_mut,
      draw_text_mut
    },
    rect::Rect
  }
};

pub enum Layer {
  Rect {
    size:     (u32, u32),
    position: (u32, u32),
    color:    Rgba<u8>
  },
  Line {
    start: (u32, u32),
    end:   (u32, u32),
    width: u32,
    color: Rgba<u8>
  },
  Text {
    size:     f32,
    position: (u32, u32),
    color:    Rgba<u8>,
    content:  String,
    font:     Font
  },
  Image {
    scale:    f32,
    position: (u32, u32),
    image:    DynamicImage
  }
}

impl Layer {
  pub fn render(
    &self,
    img: &mut RgbaImage
  ) {
    match self {
      Layer::Rect { size, position, color } => {
        let rect = Rect::at(position.0 as i32, position.1 as i32).of_size(size.0, size.1);
        draw_filled_rect_mut(img, rect, *color);
      },
      Layer::Line { start, end, width, color } => {
        let (sx, sy) = (start.0 as f32, start.1 as f32);
        let (ex, ey) = (end.0 as f32, end.1 as f32);

        for w in 0..*width {
          let offset = w as f32;
          draw_line_segment_mut(img, (sx, sy + offset), (ex, ey + offset), *color);
        }
      },
      Layer::Text {
        size,
        position,
        color,
        content,
        font
      } => {
        let scale = PxScale::from(*size);
        let font = font.to_fontarc();
        draw_text_mut(img, *color, position.0 as i32, position.1 as i32, scale, &font, content)
      },
      Layer::Image { scale, position, image } => {
        let (w, h) = image.dimensions();
        let nw = (w as f32 * scale) as u32;
        let nh = (h as f32 * scale) as u32;
        let resized = image.resize_exact(nw, nh, Lanczos3);

        overlay(img, &resized, position.0.into(), position.1.into());
      }
    }
  }
}

macro_rules! load_font {
  ($path:expr) => {{
    let path = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), $path));
    FontArc::try_from_slice(path).unwrap()
  }};
}

#[derive(Clone, Copy)]
pub enum Font {
  DejaVuSans,
  UbuntuRegular,
  UbuntuBold,
  RobotoRegular,
  RobotoBold,
  /// Load your own font of choice
  Custom(&'static str)
}

impl Font {
  pub fn to_fontarc(self) -> FontArc {
    match self {
      Font::DejaVuSans => load_font!("/fonts/DejaVuSans.ttf"),
      Font::UbuntuRegular => load_font!("/fonts/ubuntu/Ubuntu-Regular.ttf"),
      Font::UbuntuBold => load_font!("/fonts/ubuntu/Ubuntu-Bold.ttf"),
      Font::RobotoRegular => load_font!("/fonts/roboto/Roboto-Regular.ttf"),
      Font::RobotoBold => load_font!("/fonts/roboto/Roboto-Bold.ttf"),
      Font::Custom(p) => Self::from_path(p)
    }
  }

  fn from_path(path: &str) -> FontArc {
    let font = std::fs::read(path).unwrap_or_else(|_| panic!("(Asahi) failed to load font from given path at {path}"));

    FontArc::try_from_vec(font).unwrap_or_else(|_| panic!("(Asahi) failed to parse font from given path at {path}"))
  }
}
