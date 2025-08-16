use std::fs;

#[derive(Clone, Debug)]
pub struct ObjectProperties{
    pub stl_data: String,
    pub texture_data: String,
    pub normal_map: String,
    pub uv: String,
}

impl Default for ObjectProperties {
    fn default() -> Self {
        Self{
            stl_data: "Не выбрано".to_string(),
            texture_data: "Не выбрано".to_string(),
            normal_map: "Не выбрано".to_string(),
            uv: "Не выбрано".to_string(),
        }
    }
}

pub fn list_files_from_dir(dir: &str, extension: &str)->Vec<String>{
    let files = match fs::read_dir(dir) {
        Ok(entries) => {
            entries
                .filter_map(|entry| {
                    let entry = entry.ok()?;
                    let path = entry.path();
                    if path.is_file() && path.extension()?.to_str()? == extension {
                        path.file_name()?.to_str().map(|s| s.to_owned())
                    } else {
                        None
                    }
                })
                .collect::<Vec<String>>()
        }
        Err(_) => {
            Vec::new()
        }
        
    };
    
    files
}