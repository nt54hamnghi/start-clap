use clap::Parser;

mod styles;

#[derive(Parser, Debug)]
#[command(name = "{{crate_name}}")]
#[command(author, version, about)]
#[command(propagate_version = true)]
#[command(styles = styles::get_styles())]
pub struct Cli {}
