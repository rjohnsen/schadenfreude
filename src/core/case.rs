
use crate::core::config;
use std::path::Path;

pub fn create(case_name: &String) -> String {
    let folder_path = config::get_path();
    let case_path = Path::new(&folder_path).join(&case_name);

    if case_path.is_dir() {
        match std::fs::remove_dir_all(case_path.clone()) {
            Err(e) => println!("{}", e),
            Ok(()) => println!("Case folder '{}' removed", &case_name),
        }
    }

    match std::fs::create_dir(case_path) {
        Err(e) => println!("{}", e),
        Ok(()) => println!("Case folder '{}' created", &case_name),
    }

    return case_name.to_string();
}