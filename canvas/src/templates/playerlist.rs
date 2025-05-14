use {
  super::graph::{
    PlayerGraphOpts,
    graph_playercount
  },
  crate::{
    Canvas,
    Layer
  },
  ab_glyph::{
    Font,
    FontArc,
    PxScale,
    ScaleFont
  },
  image::Rgba
};

/// Player data entry
pub struct PlayerEntry {
  pub name:     String,
  pub uptime:   String,
  pub is_admin: bool,
  pub emoji:    String
}

/// Style override options
pub struct Style {
  pub bg_color:         Rgba<u8>,
  pub header_bar_color: Rgba<u8>,
  pub graph_color:      Rgba<u8>,
  pub text_color:       Rgba<u8>,
  pub admin_color:      Rgba<u8>,
  pub font:             FontArc,
  pub font_size:        f32,
  pub row_height:       u32,
  pub padding:          u32
}

impl Default for Style {
  fn default() -> Self {
    Self {
      bg_color:         Rgba([5, 5, 5, 255]),
      header_bar_color: Rgba([10, 10, 10, 255]),
      graph_color:      Rgba([201, 55, 93, 255]),
      text_color:       Rgba([255, 255, 255, 255]),
      admin_color:      Rgba([247, 67, 74, 255]),
      font:             FontArc::try_from_slice(include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/fonts/DejaVuSans.ttf"))).unwrap(),
      font_size:        24.0,
      row_height:       36,
      padding:          5
    }
  }
}

fn assume_text_width(
  text: &str,
  font_size: f32,
  font: &FontArc
) -> u32 {
  let scaled = font.as_scaled(PxScale::from(font_size));
  text.chars().map(|c| scaled.h_advance(scaled.glyph_id(c))).sum::<f32>().ceil() as u32
}

pub fn playerlist(
  players: &[PlayerEntry],
  graph_data: &[i32],
  display_graph: bool,
  style: Option<Style>
) -> Canvas {
  let style = style.unwrap_or_default();
  let width = 600;
  let graph_height = if display_graph { 120 } else { 0 };
  let header_height = 50;
  let height = header_height + players.len() as u32 * style.row_height + style.padding * 2 + graph_height;
  let mut canvas = Canvas::new(width, height);
  canvas.set_bg_color(style.bg_color);

  if players.is_empty() {
    // header
    canvas.add_layer(Layer::Rect {
      size:     (width, header_height),
      position: (0, 0),
      color:    style.header_bar_color
    });

    // header text
    let content = "Nobody playing";
    let fsize = style.font_size + 8.0;
    let text_width = assume_text_width(content, fsize, &style.font);
    let text_x = (width / 2).saturating_sub(text_width / 2);
    canvas.add_layer(Layer::Text {
      size:     fsize,
      position: (text_x, 7),
      color:    style.text_color,
      content:  content.to_string(),
      font:     style.font.clone()
    });

    return canvas
  }

  // header
  canvas.add_layer(Layer::Rect {
    size:     (width, header_height),
    position: (0, 0),
    color:    style.header_bar_color
  });

  // header text
  let content = "Players online";
  let fsize = style.font_size + 8.0;
  let text_width = assume_text_width(content, fsize, &style.font);
  let text_x = (width / 2).saturating_sub(text_width / 2);
  canvas.add_layer(Layer::Text {
    size:     fsize,
    position: (text_x, 7),
    color:    style.text_color,
    content:  content.to_string(),
    font:     style.font.clone()
  });

  for (i, p) in players.iter().enumerate() {
    let y = header_height + i as u32 * style.row_height;

    let alt_color = if i % 2 == 0 { Rgba([10, 10, 10, 255]) } else { Rgba([20, 20, 20, 255]) };

    // alternating color thing
    canvas.add_layer(Layer::Rect {
      size:     (width, style.row_height),
      position: (0, y),
      color:    alt_color
    });

    let admin_color = if p.is_admin { style.admin_color } else { style.text_color };

    // player data field
    canvas.add_layer(Layer::Text {
      size:     style.font_size,
      position: (50, y + 5),
      color:    admin_color,
      content:  format!("{}{} - {}", p.name, p.emoji, p.uptime),
      font:     style.font.clone()
    })
  }

  if display_graph {
    let opts = PlayerGraphOpts {
      line_color: style.graph_color,
      ..Default::default()
    };
    graph_playercount(&mut canvas, graph_data, opts);
  }

  canvas
}

mod test {
  #[allow(unused_imports)]
  use {
    super::{
      Canvas,
      PlayerEntry,
      playerlist
    },
    crate::ImageFormat,
    image::{
      GenericImageView,
      ImageReader
    }
  };

  #[test]
  fn test_disk_emptylist() {
    let players = [];
    let canvas = playerlist(&players, &[2, 5, 7, 10, 13, 9], true, None);
    std::fs::write(
      "playerlistempty_export_test.jpg",
      canvas.to_bytes(Some(ImageFormat::Jpeg { quality: 100 })).unwrap()
    )
    .unwrap();
  }

  #[test]
  fn test_disk() {
    let players = [
      PlayerEntry {
        name:     "Nwero".to_string(),
        uptime:   "4 h".to_string(),
        is_admin: true,
        emoji:    "🧃".to_string()
      },
      PlayerEntry {
        name:     "Test2".to_string(),
        uptime:   "3 h 51 m".to_string(),
        is_admin: false,
        emoji:    "".to_string()
      },
      PlayerEntry {
        name:     "Friendly Spider".to_string(),
        uptime:   "3 h 49 m".to_string(),
        is_admin: true,
        emoji:    "🕷️".to_string()
      },
      PlayerEntry {
        name:     "Test4567890".to_string(),
        uptime:   "3 h 40 m".to_string(),
        is_admin: false,
        emoji:    "💾".to_string()
      },
      PlayerEntry {
        name:     "TestingTesting".to_string(),
        uptime:   "3 h".to_string(),
        is_admin: false,
        emoji:    "".to_string()
      },
      PlayerEntry {
        name:     "Daggerwin".to_string(),
        uptime:   "2 h 4 m".to_string(),
        is_admin: true,
        emoji:    "".to_string()
      },
      PlayerEntry {
        name:     "Annoying Thing".to_string(),
        uptime:   "2 h".to_string(),
        is_admin: false,
        emoji:    "".to_string()
      },
      PlayerEntry {
        name:     "Mr. Pallet".to_string(),
        uptime:   "2 h".to_string(),
        is_admin: false,
        emoji:    "".to_string()
      },
      PlayerEntry {
        name:     "Daggerbot - hi".to_string(),
        uptime:   "1 h 58 m".to_string(),
        is_admin: true,
        emoji:    "".to_string()
      },
      PlayerEntry {
        name:     "Random player".to_string(),
        uptime:   "1 h 46 m".to_string(),
        is_admin: false,
        emoji:    "".to_string()
      },
      PlayerEntry {
        name:     "4220".to_string(),
        uptime:   "59 m".to_string(),
        is_admin: false,
        emoji:    "".to_string()
      },
      PlayerEntry {
        name:     "asdf".to_string(),
        uptime:   "35 m".to_string(),
        is_admin: false,
        emoji:    "".to_string()
      },
      PlayerEntry {
        name:     "Holy hell".to_string(),
        uptime:   "12 m".to_string(),
        is_admin: false,
        emoji:    "".to_string()
      },
      PlayerEntry {
        name:     "This is getting".to_string(),
        uptime:   "12 m".to_string(),
        is_admin: false,
        emoji:    "".to_string()
      },
      PlayerEntry {
        name:     "too long to type".to_string(),
        uptime:   "11 m".to_string(),
        is_admin: false,
        emoji:    "".to_string()
      },
      PlayerEntry {
        name:     "aslkfjafoijlkadfj".to_string(),
        uptime:   "1 m".to_string(),
        is_admin: false,
        emoji:    "".to_string()
      }
    ];

    let canvas = playerlist(&players, &[2, 5, 7, 10, 13, 9], true, None);
    std::fs::write(
      "playerlist_export_test.jpg",
      canvas.to_bytes(Some(ImageFormat::Jpeg { quality: 100 })).unwrap()
    )
    .unwrap();
  }

  #[test]
  fn test_bytes() {
    let players = [
      PlayerEntry {
        name:     "Nwero".to_string(),
        uptime:   "4 h".to_string(),
        is_admin: true,
        emoji:    "🧃".to_string()
      },
      PlayerEntry {
        name:     "Test2".to_string(),
        uptime:   "3 h 51 m".to_string(),
        is_admin: false,
        emoji:    "".to_string()
      },
      PlayerEntry {
        name:     "Friendly Spider".to_string(),
        uptime:   "3 h 49 m".to_string(),
        is_admin: true,
        emoji:    "🕷️".to_string()
      },
      PlayerEntry {
        name:     "Test4567890".to_string(),
        uptime:   "3 h 40 m".to_string(),
        is_admin: false,
        emoji:    "💾".to_string()
      },
      PlayerEntry {
        name:     "TestingTesting".to_string(),
        uptime:   "3 h".to_string(),
        is_admin: false,
        emoji:    "".to_string()
      }
    ];

    let canvas = playerlist(&players, &[], false, None);
    let bytes = canvas.to_bytes(None).expect("bytes export failed");

    assert!(!bytes.is_empty());

    let cursor = std::io::Cursor::new(&bytes);
    let decoded = ImageReader::new(cursor)
      .with_guessed_format()
      .expect("format guessing failed")
      .decode()
      .expect("decoding failed");

    let (w, h) = decoded.dimensions();
    assert!(w > 100 && h > 50, "image too small or none");
  }
}
