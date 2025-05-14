use {
  crate::{
    Canvas,
    Layer
  },
  image::Rgba
};

pub struct PlayerGraphOpts {
  pub max_players: u8,
  pub width:       u32,
  pub height:      u32,
  pub line_color:  Rgba<u8>,
  pub line_width:  u32
}

impl Default for PlayerGraphOpts {
  fn default() -> Self {
    Self {
      max_players: 16,
      width:       700,
      height:      100,
      line_color:  Rgba([100, 180, 255, 255]),
      line_width:  4
    }
  }
}

pub fn graph_playercount(
  canvas: &mut Canvas,
  data: &[i32],
  opts: PlayerGraphOpts
) {
  if data.len() < 2 || opts.max_players < 2 {
    return
  }

  let padding_x = canvas.width.saturating_sub(opts.width) / 2;
  let padding_bottom = 16;
  let graph_base_y = canvas.height - opts.height - padding_bottom;

  let y_scale = opts.height as f32 / opts.max_players as f32;
  let x_step = opts.width as f32 / (data.len() - 1) as f32;

  for i in 0..(data.len() - 1) {
    let p1 = &data[i];
    let p2 = &data[i + 1];

    let x1 = padding_x as f32 + i as f32 * x_step;
    let x2 = padding_x as f32 + (i + 1) as f32 * x_step;

    let y1 = graph_base_y as f32 + (opts.height as f32 - *p1 as f32 * y_scale);
    let y2 = graph_base_y as f32 + (opts.height as f32 - *p2 as f32 * y_scale);

    canvas.add_layer(Layer::Line {
      start: (x1 as u32, y1 as u32),
      end:   (x2 as u32, y2 as u32),
      width: opts.line_width,
      color: opts.line_color
    });
  }
}
