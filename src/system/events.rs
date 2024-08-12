use bevy::prelude::*;

pub enum StatusType {
    MainMenu,
    Rank,
    Game,
}

#[derive(Event)]
pub struct ChangeGameStatus(pub StatusType);

pub struct TextPopupEvent {
    pub content: String,
    pub font: Option<Handle<Font>>,
    pub font_size: f32,
    pub font_color: Color,
    pub border: UiRect,
    pub border_color: Color,
    pub padding: UiRect,
    pub margin: UiRect,
    pub modal: Option<Color>,
    pub text_alignment: JustifyText,
    pub background_color: Color,
}

impl Default for TextPopupEvent {
    fn default() -> Self {
        Self {
            content: default(),
            font: default(),
            font_size: 32.0,
            font_color: Color::WHITE,
            border: UiRect::all(Val::Px(5.)),
            border_color: Color::WHITE.with_a(0.5),
            padding: UiRect::all(Val::Px(20.)),
            margin: UiRect::all(Val::Px(10.)),
            modal: None,
            text_alignment: JustifyText::Center,
            background_color: Color::BLACK,
        }
    }
}

#[derive(Debug, Component)]
pub struct TextPopup;

#[derive(Debug, Component)]
pub struct TextPopupExpires {
    pub expires_in: f32,
}