use {
  crate::{
    canvas::Canvas,
    layer::{
      Font as LFont,
      Layer
    },
    to_rgba
  },
  asahi_utils::format_bytes,
  image::Rgba,
  std::fmt
};

#[derive(Debug, Clone)]
pub struct Metadata {
  pub name:      String,
  pub extension: String,
  pub is_folder: bool,
  /// File size in bytes, it will be hidden if `is_folder` is true
  pub size:      Option<u64>,
  /// Displayed in 'Date modified' column
  pub date:      String,
  pub icon:      FileIcon
}

#[derive(Debug, Clone, Copy)]
pub enum FileIcon {
  Folder,
  Document,
  Executable,
  Archive,
  Image,
  Video,
  Audio,
  Code,
  Unknown
}

impl Metadata {
  pub fn new_folder(
    name: String,
    date: String
  ) -> Self {
    Self {
      name,
      extension: String::new(),
      is_folder: true,
      size: None,
      date,
      icon: FileIcon::Folder
    }
  }

  pub fn new_file(
    name: String,
    date: String,
    extension: String,
    size: u64
  ) -> Self {
    let icon = FileIcon::get_icon(&extension);
    Self {
      name,
      extension,
      is_folder: false,
      size: Some(size),
      date,
      icon
    }
  }
}

impl FileIcon {
  fn get_icon(extension: &str) -> Self {
    match extension.to_ascii_lowercase().as_str() {
      "jpg" | "jpeg" | "png" | "webp" | "gif" => Self::Image,
      "mp4" | "webm" => Self::Video,
      "mp3" | "wav" | "ogg" => Self::Audio,
      "exe" | "msi" => Self::Executable,
      "zip" | "rar" | "7z" => Self::Archive,
      "txt" | "log" => Self::Document,
      "xml" | "js" | "lua" | "rs" => Self::Code,
      _ => Self::Unknown
    }
  }

  fn to_color(self) -> Rgba<u8> {
    match self {
      FileIcon::Folder => to_rgba(0xFFD700),     // Gold
      FileIcon::Document => to_rgba(0x4285F4),   // Blue
      FileIcon::Image => to_rgba(0x34A853),      // Green
      FileIcon::Video => to_rgba(0xEA4335),      // Red
      FileIcon::Audio => to_rgba(0xFF6D01),      // Orange
      FileIcon::Executable => to_rgba(0x9C27B0), // Purple
      FileIcon::Archive => to_rgba(0x795548),    // Brown
      FileIcon::Code => to_rgba(0xA7A7CA),       // Wistful
      FileIcon::Unknown => to_rgba(0x9E9E9E)     // Grey
    }
  }
}

impl fmt::Display for FileIcon {
  fn fmt(
    &self,
    f: &mut fmt::Formatter<'_>
  ) -> fmt::Result {
    let text = match self {
      FileIcon::Archive => "Archive",
      FileIcon::Audio => "Audio",
      FileIcon::Code => "Code",
      FileIcon::Document => "Text Document",
      FileIcon::Executable => "Application",
      FileIcon::Folder => "File folder",
      FileIcon::Image => "Image",
      FileIcon::Video => "Video",
      _ => "File"
    };
    write!(f, "{text}")
  }
}

/// Style override options
pub struct Style {
  pub theme:            Theme,
  pub font:             LFont,
  pub font_size:        f32,
  pub header_font_size: f32,
  pub row_height:       u32,
  pub padding:          u32
}

impl Default for Style {
  fn default() -> Self {
    Self {
      theme:            Theme::Light,
      font:             LFont::RobotoRegular,
      font_size:        12.0,
      header_font_size: 14.0,
      row_height:       22,
      padding:          8
    }
  }
}

impl Style {
  pub fn dark() -> Self {
    Self {
      theme: Theme::Dark,
      ..Default::default()
    }
  }

  fn bg_color(&self) -> Rgba<u8> {
    match self.theme {
      Theme::Light => to_rgba(0xFFFFFF),
      Theme::Dark => to_rgba(0x2B2D31)
    }
  }

  fn header_bar_color(&self) -> Rgba<u8> {
    match self.theme {
      Theme::Light => to_rgba(0xF0F0F0),
      Theme::Dark => to_rgba(0x1E1F22)
    }
  }

  fn toolbar_color(&self) -> Rgba<u8> {
    match self.theme {
      Theme::Light => to_rgba(0xF8F8F8),
      Theme::Dark => to_rgba(0x232428)
    }
  }

  fn alt_row_color(&self) -> Rgba<u8> {
    match self.theme {
      Theme::Light => to_rgba(0xF8F8F8),
      Theme::Dark => to_rgba(0x232428)
    }
  }

  fn text_color(&self) -> Rgba<u8> {
    match self.theme {
      Theme::Light => to_rgba(0x000000),
      Theme::Dark => to_rgba(0xDBDEE1)
    }
  }

  fn header_text_color(&self) -> Rgba<u8> {
    match self.theme {
      Theme::Light => to_rgba(0x000000),
      Theme::Dark => to_rgba(0xB9BBBE)
    }
  }

  fn border_color(&self) -> Rgba<u8> {
    match self.theme {
      Theme::Light => to_rgba(0xD0D0D0),
      Theme::Dark => to_rgba(0x3F4147)
    }
  }

  fn highlight_color(&self) -> Rgba<u8> {
    match self.theme {
      Theme::Light => to_rgba(0x0078D4),
      Theme::Dark => to_rgba(0x404EED)
    }
  }

  fn highlight_text_color(&self) -> Rgba<u8> { to_rgba(0xFFFFFF) }
}

/// Theme options for File Explorer
pub enum Theme {
  Light,
  Dark
}

/// Mimics the Windows 10 File Explorer design
pub fn file_explorer(
  current_path: &str,
  files: &[Metadata],
  width: u32,
  show_back_btn: bool,
  highlighted_index: Option<usize>,
  style: Option<Style>
) -> Canvas {
  let style = style.unwrap_or_default();

  // view heights
  const HEADER_HEIGHT: u32 = 40;
  const TOOLBAR_HEIGHT: u32 = 35;
  const COLUMN_HEADER_HEIGHT: u32 = 24;
  const ICON_SIZE: u32 = 16;

  let total_height = HEADER_HEIGHT + TOOLBAR_HEIGHT + COLUMN_HEADER_HEIGHT + (files.len() as u32 * style.row_height) + style.padding * 2;

  let mut canvas = Canvas::new(width, total_height);
  canvas.set_bg_color(style.bg_color());

  // titlebar
  canvas.add_layer(Layer::Rect {
    size:     (width, HEADER_HEIGHT),
    position: (0, 0),
    color:    style.header_bar_color()
  });

  canvas.add_layer(Layer::Text {
    size:     style.header_font_size,
    position: (style.padding, 12),
    color:    style.header_text_color(),
    content:  "File Explorer".to_string(),
    font:     style.font
  });

  // toolbar bg
  canvas.add_layer(Layer::Rect {
    size:     (width, TOOLBAR_HEIGHT),
    position: (0, HEADER_HEIGHT),
    color:    style.toolbar_color()
  });

  // back button
  if show_back_btn {
    canvas.add_layer(Layer::Rect {
      size:     (60, 25),
      position: (style.padding, HEADER_HEIGHT + 5),
      color:    style.border_color()
    });

    canvas.add_layer(Layer::Text {
      size:     style.font_size,
      position: (style.padding + 15, HEADER_HEIGHT + 12),
      color:    style.text_color(),
      content:  "Back".to_string(),
      font:     style.font
    });
  }

  // address bar
  let address_x = if show_back_btn { 80 } else { style.padding };
  let address_w = width - address_x - style.padding;
  canvas.add_layer(Layer::Rect {
    size:     (address_w, 25),
    position: (address_x, HEADER_HEIGHT + 5),
    color:    style.bg_color()
  });

  // address bar's border
  canvas.add_layer(Layer::Line {
    start: (address_x, HEADER_HEIGHT + 5),
    end:   (address_x + address_w, HEADER_HEIGHT + 5),
    width: 1,
    color: style.border_color()
  });

  canvas.add_layer(Layer::Line {
    start: (address_x, HEADER_HEIGHT + 30),
    end:   (address_x + address_w, HEADER_HEIGHT + 30),
    width: 1,
    color: style.border_color()
  });

  canvas.add_layer(Layer::Text {
    size:     style.font_size,
    position: (address_x + 5, HEADER_HEIGHT + 12),
    color:    style.text_color(),
    content:  current_path.to_string(),
    font:     style.font
  });

  // column headers
  let header_y = HEADER_HEIGHT + TOOLBAR_HEIGHT;
  canvas.add_layer(Layer::Rect {
    size:     (width, COLUMN_HEADER_HEIGHT),
    position: (0, header_y),
    color:    style.header_bar_color()
  });

  // column widths
  let name_w = width * 40 / 100;
  let date_w = width * 30 / 100;
  let type_w = width * 20 / 100;

  // column header texts
  canvas.add_layer(Layer::Text {
    size:     style.font_size,
    position: (style.padding + ICON_SIZE + 5, header_y + 6),
    color:    style.header_text_color(),
    content:  "Name".to_string(),
    font:     style.font
  });

  canvas.add_layer(Layer::Text {
    size:     style.font_size,
    position: (name_w + 5, header_y + 6),
    color:    style.header_text_color(),
    content:  "Date modified".to_string(),
    font:     style.font
  });

  canvas.add_layer(Layer::Text {
    size:     style.font_size,
    position: (name_w + date_w + 5, header_y + 6),
    color:    style.header_text_color(),
    content:  "Type".to_string(),
    font:     style.font
  });

  canvas.add_layer(Layer::Text {
    size:     style.font_size,
    position: (name_w + date_w + type_w + 5, header_y + 6),
    color:    style.header_text_color(),
    content:  "Size".to_string(),
    font:     style.font
  });

  // column separators
  canvas.add_layer(Layer::Line {
    start: (name_w, header_y),
    end:   (name_w, header_y + COLUMN_HEADER_HEIGHT),
    width: 1,
    color: style.border_color()
  });

  canvas.add_layer(Layer::Line {
    start: (name_w + date_w, header_y),
    end:   (name_w + date_w, header_y + COLUMN_HEADER_HEIGHT),
    width: 1,
    color: style.border_color()
  });

  canvas.add_layer(Layer::Line {
    start: (name_w + date_w + type_w, header_y),
    end:   (name_w + date_w + type_w, header_y + COLUMN_HEADER_HEIGHT),
    width: 1,
    color: style.border_color()
  });

  // rows for file/folder
  let content_start_y = header_y + COLUMN_HEADER_HEIGHT;
  for (i, f) in files.iter().enumerate() {
    let row_y = content_start_y + (i as u32 * style.row_height);
    let is_highlighted = highlighted_index == Some(i);

    // row bg
    let (row_color, text_color) = if is_highlighted {
      (style.highlight_color(), style.highlight_text_color())
    } else if i % 2 == 1 {
      (style.alt_row_color(), style.text_color())
    } else {
      (style.bg_color(), style.text_color())
    };

    canvas.add_layer(Layer::Rect {
      size:     (width, style.row_height),
      position: (0, row_y),
      color:    row_color
    });

    // icon
    canvas.add_layer(Layer::Rect {
      size:     (ICON_SIZE, ICON_SIZE),
      position: (style.padding, row_y + 3),
      color:    f.icon.to_color()
    });

    // name
    let name = if f.extension.is_empty() || f.is_folder {
      f.name.clone()
    } else {
      format!("{}.{}", f.name, f.extension)
    };

    canvas.add_layer(Layer::Text {
      size:     style.font_size,
      position: (style.padding + ICON_SIZE + 5, row_y + 5),
      color:    text_color,
      content:  name,
      font:     style.font
    });

    // date modified text
    canvas.add_layer(Layer::Text {
      size:     style.font_size,
      position: (name_w + 5, row_y + 5),
      color:    text_color,
      content:  f.date.clone(),
      font:     style.font
    });

    // type text
    canvas.add_layer(Layer::Text {
      size:     style.font_size,
      position: (name_w + date_w + 5, row_y + 5),
      color:    text_color,
      content:  f.icon.to_string(),
      font:     style.font
    });

    // size text
    let size_text = if let Some(size) = f.size {
      format_bytes(size)
    } else {
      String::new() // for folders
    };

    canvas.add_layer(Layer::Text {
      size:     style.font_size,
      position: (name_w + date_w + type_w + 5, row_y + 5),
      color:    text_color,
      content:  size_text,
      font:     style.font
    });
  }

  canvas
}

#[cfg(test)]
mod test {
  use {
    super::*,
    crate::ImageFormat
  };

  #[test]
  fn test_file_explorer_empty() {
    let files = [];
    let canvas = file_explorer("C:\\Users\\asahi\\Desktop", &files, 600, false, None, None);
    std::fs::write(
      "file_explorer_empty_test.jpg",
      canvas.to_bytes(Some(ImageFormat::Jpeg { quality: 100 })).unwrap()
    )
    .unwrap();
  }

  #[test]
  fn test_file_explorer_with_files() {
    let files = [
      Metadata::new_folder("New folder (9)".to_string(), "17/02/2025 18:30".to_string()),
      Metadata::new_file("gigi-murin".to_string(), "03/06/2025 11:07".to_string(), "png".to_string(), 10485760),
      Metadata::new_file("gigi-murin2".to_string(), "03/06/2025 11:12".to_string(), "jpg".to_string(), 12582912)
    ];

    let canvas = file_explorer(
      "G:\\",
      &files,
      600,
      true,
      Some(8),
      Some(Style {
        theme: Theme::Dark,
        ..Default::default()
      })
    );
    std::fs::write(
      "file_explorer_test.jpg",
      canvas.to_bytes(Some(ImageFormat::Jpeg { quality: 100 })).unwrap()
    )
    .unwrap();
  }

  #[test]
  fn test_file_explorer_bytes() {
    let files = [
      Metadata::new_folder("Folder1".to_string(), "12/10/2024 03:01".to_string()),
      Metadata::new_file("test4".to_string(), "29/07/2019 12:21".to_string(), "txt".to_string(), 8192)
    ];
    let canvas = file_explorer("C:\\Test", &files, 600, false, None, None);
    std::fs::write(
      "file_explorer_test_bytes.jpg",
      canvas.to_bytes(Some(ImageFormat::Jpeg { quality: 100 })).unwrap()
    )
    .unwrap();
  }
}
