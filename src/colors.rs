use tui::style::{Color, Style};

pub const HN_BACKGROUND: Color = Color::Rgb(246,246,239);
pub const HN_ORANGE: Color = Color::Rgb(255,102,0);

pub enum HNStyles {
    OrangeBlock,
    WhiteBlock,
    OrangeBorder,
    OrangeTitle,
    WhiteTitle,
}

pub fn get_style(style: HNStyles) -> Style {
    match style {
        HNStyles::OrangeBlock => Style::default().bg(HN_ORANGE).fg(HN_BACKGROUND),
        HNStyles::WhiteBlock => Style::default().bg(HN_BACKGROUND).fg(Color::Black),
        HNStyles::OrangeBorder => Style::default().bg(HN_BACKGROUND).fg(HN_ORANGE),
        HNStyles::OrangeTitle => Style::default().bg(HN_ORANGE).fg(HN_BACKGROUND),
        HNStyles::WhiteTitle => Style::default().bg(HN_BACKGROUND).fg(HN_ORANGE),
    }
}

