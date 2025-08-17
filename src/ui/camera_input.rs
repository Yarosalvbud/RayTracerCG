use crate::ui::move_input::UserMove;

#[derive(Clone, Debug)]
pub struct CameraCommand {
    pub translation: UserMove,
    pub rotation: UserMove,
    pub fov: String,
}

impl Default for CameraCommand {
    fn default() -> Self {
        Self{
            translation: UserMove::default(),
            rotation: UserMove::default(),
            fov: String::new(),
        }
    }
}