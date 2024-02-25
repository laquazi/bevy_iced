use bevy::prelude::*;
use bevy_iced::iced::widget::{column, text};
use bevy_iced::iced::{font, Font, Pixels};
use bevy_iced::{iced, IcedContext, IcedPlugin};

const ALPHAPROTA_FONT: Font = Font::with_name("Alpha Prota");
const ALPHAPROTA_FONT_BYTES: &'static [u8] = include_bytes!("../assets/fonts/AlphaProta.ttf");

#[derive(Event)]
pub enum UiMessage {}

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(IcedPlugin {
            fonts: vec![ALPHAPROTA_FONT_BYTES],
            settings: iced::Settings {
                default_text_size: Pixels(40.0),
                default_font: ALPHAPROTA_FONT,
                ..Default::default()
            },
        })
        .add_event::<UiMessage>()
        .add_systems(Update, ui_system)
        .run();
}

fn ui_system(mut ctx: IcedContext<UiMessage>) {
    ctx.display(column!(
        text(format!("I am the default font")).font(font::Font::DEFAULT),
        text(format!("I am another font"))
    ));
}
