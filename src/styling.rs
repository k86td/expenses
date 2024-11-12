use tabled::{
    settings::{object::Rows, style::BorderColor, themes::Colorization, Color, Style},
    Table,
};

const ESC_CHAR: &str = "\x1b[";

/// Check [this](https://en.wikipedia.org/wiki/ANSI_escape_code#SGR) wikipedia page about it
pub enum AsciiStyling<'a> {
    Reset,
    Bold,
    Underline,
    BackgroundColor8Bit(&'a str),
}

impl ToString for AsciiStyling<'_> {
    fn to_string(&self) -> String {
        format!(
            "{}{}",
            ESC_CHAR,
            match self {
                AsciiStyling::Reset => "0m".to_owned(),
                AsciiStyling::Bold => "3m".to_owned(),
                AsciiStyling::Underline => "4m".to_owned(),
                AsciiStyling::BackgroundColor8Bit(bg) => format!("48;5;{}", &bg).to_owned(),
            }
        )
    }
}

impl From<AsciiStyling<'_>> for String {
    fn from(value: AsciiStyling) -> Self {
        value.to_string()
    }
}

// TODO: update this with tabled version including `Rows::new(...).step_by(2)`
pub fn new_table<I, T>(iter: I) -> tabled::Table
where
    I: IntoIterator<Item = T>,
    T: tabled::Tabled,
{
    let underline_style = Color::new(AsciiStyling::Underline, AsciiStyling::Reset);
    let reset_style = Color::new(AsciiStyling::Reset, AsciiStyling::Reset);
    let mut table = Table::new(iter);
    table
        .with(Style::blank())
        .with(Colorization::rows([reset_style.clone(), Color::BG_BLACK]))
        .with(Colorization::exact([underline_style], Rows::first()));

    for row in (2..table.count_rows()).step_by(2) {
        table.modify(
            Rows::single(row),
            BorderColor::new().left(reset_style.clone()),
        );
    }

    for row in (1..table.count_rows()).step_by(2) {
        table.modify(Rows::single(row), BorderColor::new().left(Color::BG_BLACK));
    }

    table
}
