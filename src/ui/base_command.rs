use crate::ui::errors::UiError;

pub mod object_commands;
pub mod light_commands;
pub mod camera_commands;
pub mod render_command;

pub trait Command{
    fn execute(&mut self) -> Result<(), UiError>;
}



