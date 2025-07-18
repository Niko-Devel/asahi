use {
  super::graph::{
    PlayerGraphOpts,
    graph_playercount
  },
  crate::{
    canvas::{
      Canvas,
      EmoteSource,
      assume_text_width,
      parse_all_emotes
    },
    layer::{
      Font as LFont,
      Layer
    },
    worker::EMOTE_FETCHER_TX
  },
  image::{
    DynamicImage,
    Rgba
  },
  std::{
    collections::HashMap,
    sync::{
      LazyLock,
      Mutex
    }
  }
};

/// Caches the fetched images to avoid redownloading them
pub(crate) static DISCORD_EMOTES_CACHE: LazyLock<Mutex<HashMap<String, DynamicImage>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

/// Player data entry
pub struct PlayerEntry {
  pub name:     String,
  pub uptime:   String,
  pub is_admin: bool,
  pub emoji:    String
}

/// Style override options
pub struct Style {
  /// Text to display if playerlist is empty<br>
  /// Defaults to **Nobody playing**
  pub playerlist_empty:      String,
  /// Download and bake Discord emotes into the image<br>
  /// Defaults to `false`
  pub render_discord_emotes: bool,
  pub bg_color:              Rgba<u8>,
  pub header_bar_color:      Rgba<u8>,
  pub graph_color:           Rgba<u8>,
  pub text_color:            Rgba<u8>,
  pub admin_color:           Rgba<u8>,
  pub font:                  LFont,
  pub font_size:             f32,
  pub row_height:            u32,
  pub padding:               u32
}

impl Default for Style {
  fn default() -> Self {
    Self {
      playerlist_empty:      String::from("Nobody playing"),
      render_discord_emotes: false,
      bg_color:              Rgba([5, 5, 5, 255]),
      header_bar_color:      Rgba([10, 10, 10, 255]),
      graph_color:           Rgba([201, 55, 93, 255]),
      text_color:            Rgba([255, 255, 255, 255]),
      admin_color:           Rgba([247, 67, 74, 255]),
      font:                  LFont::UbuntuRegular,
      font_size:             24.0,
      row_height:            36,
      padding:               5
    }
  }
}

pub fn playerlist(
  players: &[PlayerEntry],
  graph_data: &[i32],
  display_graph: bool,
  style: Option<Style>
) -> Canvas {
  let style = style.unwrap_or_default();
  let discord_emotes_cache = DISCORD_EMOTES_CACHE.lock().expect("failed to acquire lock");

  let width = 600;
  let graph_height = if display_graph { 120 } else { 0 };
  let header_height = 50;
  let height = header_height + players.len() as u32 * style.row_height + style.padding * 2 + graph_height;
  let mut canvas = Canvas::new(width, height);
  canvas.set_bg_color(style.bg_color);

  // header
  canvas.add_layer(Layer::Rect {
    size:     (width, header_height),
    position: (0, 0),
    color:    style.header_bar_color
  });

  // header text
  let content = if players.is_empty() {
    style.playerlist_empty
  } else {
    "Players online".to_string()
  };
  let fsize = style.font_size + 8.0;
  let text_width = assume_text_width(&content, fsize, &style.font.to_fontarc());
  let text_x = (width / 2).saturating_sub(text_width / 2);
  canvas.add_layer(Layer::Text {
    size:     fsize,
    position: (text_x, 7),
    color:    style.text_color,
    content:  content.to_string(),
    font:     style.font
  });

  if !players.is_empty() {
    for (i, p) in players.iter().enumerate() {
      let mut x = 50;
      let y = header_height + i as u32 * style.row_height;

      let alt_color = if i.is_multiple_of(2) {
        Rgba([10, 10, 10, 255])
      } else {
        Rgba([20, 20, 20, 255])
      };

      canvas.add_layer(Layer::Rect {
        size:     (width, style.row_height),
        position: (0, y),
        color:    alt_color
      });

      let admin_color = if p.is_admin { style.admin_color } else { style.text_color };
      let uptime = if p.uptime.is_empty() {
        "Just joined".to_string()
      } else {
        p.uptime.clone()
      };

      // do player name
      canvas.add_layer(Layer::Text {
        size:     style.font_size,
        position: (x, y + 5),
        color:    admin_color,
        content:  p.name.clone(),
        font:     style.font
      });

      x += assume_text_width(&p.name, style.font_size, &style.font.to_fontarc());

      // render emotes after name
      let mut rendered = 0;
      let mut emoji_x = x;

      if style.render_discord_emotes && !p.emoji.is_empty() {
        for emoji in parse_all_emotes(&p.emoji) {
          if rendered >= 3 {
            break;
          }

          let img_opt = match emoji {
            EmoteSource::Discord(ref id) => discord_emotes_cache.get(id).cloned(),
            EmoteSource::Unicode(ch) => {
              let key = format!("twemoji_{}", ch as u32);
              discord_emotes_cache.get(&key).cloned()
            }
          };

          // if not in cache, notify worker to fetch it from internet
          if img_opt.is_none() {
            let _ = EMOTE_FETCHER_TX.send(emoji.clone());
          }

          if let Some(img) = img_opt {
            let base_px = match emoji {
              EmoteSource::Discord(_) => 96.0,
              EmoteSource::Unicode(_) => 72.0
            };

            let emote_scale = style.font_size / 72.0;
            let emote_y = y + ((style.row_height - (style.font_size as u32)) / 2).saturating_sub(2);

            canvas.add_layer(Layer::Image {
              image:    img,
              scale:    emote_scale,
              position: (emoji_x, emote_y)
            });

            emoji_x += (emote_scale * base_px) as u32 + 4;
          } else if let EmoteSource::Unicode(ch) = emoji {
            canvas.add_layer(Layer::Text {
              size:     style.font_size,
              position: (emoji_x, y + 5),
              color:    admin_color,
              content:  ch.to_string(),
              font:     style.font
            });
            emoji_x += assume_text_width(&ch.to_string(), style.font_size, &style.font.to_fontarc());
          }

          rendered += 1;
        }
      }

      // do uptime
      canvas.add_layer(Layer::Text {
        size:     style.font_size,
        position: (emoji_x, y + 5),
        color:    admin_color,
        content:  format!(" - {uptime}"),
        font:     style.font
      });
    }
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
        emoji:    "<:minyanstare:1277044182527246358> <:minyangy:1277044022430531614>".to_string()
      },
      PlayerEntry {
        name:     "Test2".to_string(),
        uptime:   "3 h 51 m".to_string(),
        is_admin: false,
        emoji:    "<:touchgrass:1007748573552726146> <:evilLookUp:1327357088959299676> <:PIPES:1372876799742312539>".to_string()
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
        emoji:    ":floppy_disk:".to_string()
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
        uptime:   "".to_string(),
        is_admin: false,
        emoji:    "".to_string()
      }
    ];

    let canvas = playerlist(
      &players,
      &[2, 5, 7, 10, 13, 9],
      true,
      Some(super::Style {
        render_discord_emotes: true,
        ..Default::default()
      })
    );
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
