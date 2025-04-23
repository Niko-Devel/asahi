use proc_macro::TokenStream;

/// Generate the impl of `AsahiPlugin`
#[proc_macro_attribute]
pub fn plugin(
  _attr: TokenStream,
  item: TokenStream
) -> TokenStream {
  let input = item.to_string();

  let module_name = input
    .lines()
    .find(|l| l.trim_start().starts_with("mod "))
    .and_then(|l| l.split_whitespace().nth(1))
    .unwrap_or("plugin")
    .trim_end_matches(";");

  let struct_name = format!("{}Plugin", module_name[..1].to_uppercase() + &module_name[1..]);

  let wrapper = format!(
    "{input}
    
    pub struct {struct_name};

    impl ::asahi::AsahiPlugin for {struct_name} {{
      fn name(&self) -> &'static str {{
        {module_name}::name()
      }}

      fn setup(&self) -> ::asahi::AsahiResult<()> {{
        {module_name}::setuo();
        Ok(())
      }}
    }}"
  );

  wrapper.parse().unwrap()
}

/// Export all plugin impl's into static slice
#[proc_macro]
pub fn export(input: TokenStream) -> TokenStream {
  let args = input.to_string();
  let tokens = format!(
    "pub static ASAHI_PLUGINS: &[&dyn ::asahi::AsahiPlugin] = &[{}];",
    args.split(',').map(|s| format!("&{}", s.trim())).collect::<Vec<_>>().join(", ")
  );
  tokens.parse().unwrap()
}
