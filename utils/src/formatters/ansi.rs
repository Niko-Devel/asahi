#[macro_export]
macro_rules! define_color_styles {
  ($name:ident, $code:expr) => {
    pub struct $name;

    impl $name {
      pub const BOLD: $crate::ansi::styling::PublicStyle = $crate::ansi::styling::PublicStyle($crate::ansi::styling::StyledColor {
        color:  $code,
        styles: &["\x1b[1m"]
      });
      pub const NORMAL: $crate::ansi::styling::PublicStyle =
        $crate::ansi::styling::PublicStyle($crate::ansi::styling::StyledColor { color: $code, styles: &[] });
    }
  };
}

define_color_styles!(Red, "\x1b[31m");
define_color_styles!(Blue, "\x1b[34m");
define_color_styles!(Yellow, "\x1b[33m");
define_color_styles!(Green, "\x1b[32m");
define_color_styles!(Black, "\x1b[37m");

mod styling {
  pub(crate) struct StyledColor<'a> {
    pub(crate) color:  &'a str,
    pub(crate) styles: &'a [&'a str]
  }

  impl<'a> StyledColor<'a> {
    pub(crate) fn paint(
      &self,
      text: &str
    ) -> String {
      let mut out = String::new();
      for s in self.styles {
        out.push_str(s);
      }
      out.push_str(self.color);
      out.push_str(text);
      out.push_str("\x1b[0m");
      out
    }
  }

  pub struct PublicStyle(pub(crate) StyledColor<'static>);

  impl PublicStyle {
    pub fn paint(
      &self,
      text: &str
    ) -> String {
      self.0.paint(text)
    }
  }
}
