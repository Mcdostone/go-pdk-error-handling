use extism::*;

fn main() {

  let url = Wasm::file("./plugin.wasm");
  let manifest = Manifest::new([url]);
  let mut plugin = Plugin::new(&manifest, [], true).unwrap();
  let res = plugin.call::<&str, &str>("parse_parameters", &serde_json::to_string(&vec!("my-input")).unwrap()).unwrap();
  println!("{}", res);
}