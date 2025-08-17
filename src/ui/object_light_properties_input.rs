use eframe::epaint::Color32;

#[derive(Clone, Debug)]
pub struct ObjectLightProperties {
    pub kd: [f32; 3],
    pub ks: [f32; 3],
    pub kt: [f32; 3],
    pub luminosity: String,
    pub object_color: Color32,
    pub background: Color32,
}


impl Default for ObjectLightProperties {
    fn default() -> ObjectLightProperties {
        ObjectLightProperties{
            kd: [1.0, 1.0, 1.0],
            ks: [0.0, 0.0, 0.0],
            kt: [0.0, 0.0, 0.0],
            luminosity: String::new(),
            object_color: Color32::from_rgb(192, 192, 192),
            background: Color32::from_rgb(0, 0, 0),
        }
    }
}