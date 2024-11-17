use std::path::Path;

pub fn change_to_config_path() {
    let platform = match std::env::consts::OS {
        "windows" => "USERPROFILE",
        "linux" => "HOME",
        "macos" => "HOME",
        _ => {
            println!("{}", std::env::consts::OS);
            panic!("Unsupported platform")
        }
    };

    let home_path = std::env::var_os(platform).expect("Failed to get home directory");
    let path = Path::new(&home_path).join(".handler");

    if !path.clone().exists() {
        let _ = std::fs::create_dir::<_>(path.clone());
    }

    //change the current directory to the config directory
    let _ = std::env::set_current_dir(path.clone());
}
