use amethyst::{
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, ReadExpect, System, SystemData, World, Write, WriteStorage},
    ui::UiText,
};

use crate::state::MenuText;

#[derive(SystemDesc)]
pub struct TextSystem;

impl<'s> System<'s> for TextSystem {
    type SystemData = (
        WriteStorage<'s, UiText>,
        ReadExpect<'s, MenuText>,
    );

    fn run(&mut self, (
        mut ui_text,
        menu_text
    ): Self::SystemData) {
        if let Some(text) = ui_text.get_mut(menu_text.text) {
            text.text = "hello".to_string();
        }
    }   
}