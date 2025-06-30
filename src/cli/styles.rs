#[allow(clippy::enum_glob_use)]
use anstyle::AnsiColor::{self, *};
use anstyle::{Color, Style};

/// Helper function to create Option<Color> to pass in builder methods of anstyle
#[allow(clippy::unnecessary_wraps)]
fn colorize(color: AnsiColor) -> Option<Color> {
    Some(color.into())
}

// https://stackoverflow.com/a/76916424
pub fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .usage(Style::new().bold().underline().fg_color(colorize(Cyan)))
        .header(Style::new().bold().underline().fg_color(colorize(Cyan)))
        .literal(Style::new().fg_color(colorize(Green)))
        .invalid(Style::new().bold().fg_color(colorize(Red)))
        .error(Style::new().bold().fg_color(colorize(Red)))
        .valid(Style::new().bold().underline().fg_color(colorize(Green)))
        .placeholder(Style::new().fg_color(colorize(Blue)))
}
