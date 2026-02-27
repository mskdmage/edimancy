mod stream_parser;
pub use stream_parser::{StreamParser, ParserConfig};

pub fn parser_config_from_isa_header(
    isa_bytes: &[u8],
) -> crate::result::Result<ParserConfig> {
    stream_parser::ParserConfig::from_isa_header(isa_bytes)
}