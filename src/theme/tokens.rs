pub mod colors {
    use iced::{Color, color};

    pub const GRAY: Color = color!(0xAAAAAA);
}

pub mod fonts {
    use iced::Font;
    pub const INTER: Font = Font::with_name("Inter 18pt");
}

pub mod radius {
    pub const SMALL: f32 = 8.0;
}

pub mod spacing {
    pub const SM: f32 = 4.0;
    pub const MD: f32 = 16.0;
    pub const XL: f32 = 32.0;
}
