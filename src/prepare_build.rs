use colored::Colorize;
use std::fs;
use std::path::PathBuf;
use crate::utils::fs_recursive_copy::copy;
use crate::utils::config::parse_config;

/// This function gathers all files from resources and
/// src directories, and transfers them in build/proj,
/// where it will be built by monkeyc.
pub fn construct_connectiq_project(manifest: String) {
    let _ = fs::create_dir("build");
    let _ = fs::create_dir("build/tmp");
    let _ = fs::create_dir("build/tmp/source");
    let _ = fs::create_dir("build/tmp/resources");

    println!("{}", "Copying source code...".bold());

    let _ = fs::File::create(PathBuf::from("build/tmp/manifest.xml"));
    let _ = fs::write(PathBuf::from("build/tmp/manifest.xml"), manifest);

    let _ = fs::File::create(PathBuf::from("build/tmp/monkey.jungle"));
    let _ = fs::write(PathBuf::from("build/tmp/monkey.jungle"), r#"project.manifest = manifest.xml"#);

    let _ = copy(PathBuf::from("src"), PathBuf::from("build/tmp/source"));
    println!("{}", "Preparing resources...".bold());
    let mut device_specific_res: Vec<String> = Vec::new();

    // Here we get all device-specific resources
    for resource in vec!["resources/drawables", "resources/layouts", "resources/fonts", "resources/menus", "resources/settings"] {
        for entry in fs::read_dir(PathBuf::from(resource)) {
            for entry in entry {
                let entry = entry.unwrap();
                if entry.file_type().unwrap().is_dir() {
                    if !device_specific_res.contains(&entry.file_name().to_str().unwrap().to_string()) {
                        device_specific_res.push(entry.file_name().to_str().unwrap().to_string());
                    }
                }
            }
        }
    }

    // And create directories
    for dir in &device_specific_res {
        let mut end_dir = PathBuf::from("build/tmp");
        let mut end_dirname: String = "resources-".parse().unwrap();
        end_dirname.push_str(&*dir);
        end_dir.push(end_dirname);
        let _ = fs::create_dir(end_dir);
    }

    // Then create directories for language resources and transfer them
    for language in parse_config(fs::read_to_string("kumitateru.toml").unwrap()).package_meta.languages {
        if language == "eng" {
            let mut end_dir = PathBuf::from("build/tmp");
            let end_dirname: String = "resources".parse().unwrap();
            end_dir.push(end_dirname);
            end_dir.push("strings");
            let _ = fs::create_dir(&end_dir);

            let mut start_directory = PathBuf::from("resources/strings");
            start_directory.push("main");

            copy(start_directory, end_dir);
        } else {
            let mut end_dir = PathBuf::from("build/tmp");
            let mut end_dirname: String = "resources-".parse().unwrap();
            end_dirname.push_str(&*language);
            end_dir.push(end_dirname);
            let _ = fs::create_dir(&end_dir);

            let mut start_directory = PathBuf::from("resources/strings");
            start_directory.push(language);

            copy(start_directory, end_dir);
        }
    }

    // And here we will transfer other resources
    for resource in vec!["drawables", "layouts", "fonts", "menus", "settings"] {
        transfer_device_resources(resource.to_string(), device_specific_res.clone());
    }
}

fn transfer_device_resources(resource: String, device_specific_res: Vec<String>) {
    for res_entry in device_specific_res {
        let mut res_dir = PathBuf::new();
        res_dir.push("resources");
        res_dir.push(&resource);
        res_dir.push(&res_entry);
        if res_dir.exists() {
            let mut end_dir = PathBuf::from("build/tmp");
            let mut end_dirname = String::from("resources-");
            end_dirname.push_str(&*res_entry);
            end_dir.push(end_dirname);
            end_dir.push(&resource);

            copy(res_dir, &end_dir);
        }
    }

}
