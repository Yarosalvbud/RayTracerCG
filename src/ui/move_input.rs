use nalgebra::{Vector3,};
use crate::ui::errors::UiError;

#[derive(Debug, Clone)]
pub struct Move{
    pub move_data: Vector3<f32>,
}

#[derive(Debug, Clone)]
pub struct UserMove{
    pub dx: String,
    pub dy: String,
    pub dz: String,
}

#[derive(Clone, Debug)]
pub struct ObjectMove{
    pub translation: UserMove,
    pub rotation: UserMove,
    pub scale: UserMove,
}

impl Default for ObjectMove {
    fn default() -> ObjectMove {
        ObjectMove{
           translation: UserMove::default(),
            rotation: UserMove::default(),
            scale: UserMove::default(),
        }
    }
}

impl Default for UserMove{
    fn default() -> Self{
        Self{
            dx: "0".to_string(),
            dy: "0".to_string(),
            dz: "0".to_string(),
        }
    }
}

impl Move{
    pub fn new(move_data: Vector3<f32>) -> Self{
        Self{
            move_data
        }
    }
    pub fn parse_from_string(user_move: &UserMove)->Result<Move, UiError>{
        if user_move.dx.is_empty() ||  user_move.dy.is_empty() || user_move.dz.is_empty(){
            return Err(UiError::MoveDataFormatError);
        }
        
        let x: Result<f32, std::num::ParseFloatError> = user_move.dx.parse();
        let y: Result<f32, std::num::ParseFloatError> = user_move.dy.parse();
        let z: Result<f32, std::num::ParseFloatError> = user_move.dz.parse();
        
        if let (Ok(x), Ok(y), Ok(z)) = (x,y,z){
            Ok(Move::new(Vector3::new(x, y, z)))
        }else{
            Err(UiError::MoveDataFormatError)
        }
    }
}