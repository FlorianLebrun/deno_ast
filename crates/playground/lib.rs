extern crate deno_ast;

use deno_ast::parse_module;
use deno_ast::MediaType;
use deno_ast::ParseParams;
use deno_ast::ParsedSource;
use deno_ast::SourceTextInfo;

struct ModuleManifold {
  sources: Vec<ParsedSource>,
}

impl ModuleManifold {
  fn add_source(&self ) {
  }
  fn new() -> ModuleManifold {
    return ModuleManifold { sources: vec![] };
  }
}

fn main() {
  let m = ModuleManifold::new();
  let source_text = "import x from 'hello'\nexports.MyClass= {}";
  let text_info = SourceTextInfo::new(source_text.into());
  let parsed_source = parse_module(ParseParams {
    specifier: "file:///my_file.ts".to_string(),
    media_type: MediaType::JavaScript,
    text_info,
    capture_tokens: true,
    maybe_syntax: None,
    scope_analysis: false,
  })
  .expect("should parse");

  // returns the comments
  parsed_source.comments();
  // returns the tokens if captured
  parsed_source.tokens();
  // returns the module (AST)
  parsed_source.module();
  // returns the `SourceTextInfo`
  parsed_source.text_info();

  let cjs = parsed_source.analyze_cjs();

  println!("exports {0}", cjs.exports.len())
}
