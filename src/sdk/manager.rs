use std::env;
use std::process::Command;

use crate::sdk::{AvailablePackage, InstalledPackage, Package, Version};

pub fn list_packages() -> Vec<Package> {
    let output = load_list();

    let (installed, available) = parse_lines(output);

    let result: Vec<Package> = available
        .iter()
        .map(|x| {
            let installed_package = installed.iter().filter(|z| z.path == z.path).nth(0);

            let loc: Option<String>;
            let installed_version: Option<Version>;
            let update_package: Option<Version>;

            match installed_package {
                Some(value) => {
                    loc = Some(value.location.clone());
                    installed_version = Some(value.version.clone());

                    if value.version < x.version {
                        update_package = Some(x.version.clone());
                    } else {
                        update_package = None;
                    }
                }
                None => {
                    loc = None;
                    installed_version = None;
                    update_package = None;
                }
            };

            Package::new(
                x.path.clone(),
                x.description.clone(),
                loc,
                installed_version,
                Some(x.version.clone()),
                update_package,
            )
        })
        .collect();

    result
}

fn parse_lines(output: String) -> (Vec<InstalledPackage>, Vec<AvailablePackage>) {
    let mut parsing_installed = false;
    let mut parsing_available = false;
    let mut parsing_updates = false;

    let mut header_skipped = false;
    let mut table_head_skipped = false;

    let mut installed: Vec<InstalledPackage> = Vec::new();
    let mut available: Vec<AvailablePackage> = Vec::new();

    let lines: Vec<&str> = output.lines().collect();

    for line in lines {
        if line.is_empty() {
            parsing_installed = false;
            parsing_available = false;
            parsing_updates = false;

            header_skipped = false;
            table_head_skipped = false;
            continue;
        }

        let ln = line.trim();

        if ln.contains("Installed packages") {
            parsing_installed = true;
            parsing_available = false;
            parsing_updates = false;

            header_skipped = false;
            table_head_skipped = false;
            continue;
        } else if ln.contains("Available Packages:") {
            parsing_installed = false;
            parsing_available = true;
            parsing_updates = false;

            header_skipped = false;
            table_head_skipped = false;
            continue;
        }

        // пропуск заголовков таблицы
        if parsing_installed || parsing_available || parsing_updates {
            if !header_skipped {
                header_skipped = true;
                continue;
            } else {
                if !table_head_skipped {
                    table_head_skipped = true;
                    continue;
                }
            }
        }

        let splits: Vec<&str> = ln.split("|").collect();

        if parsing_installed {
            assert_eq!(4, splits.len());
            installed.push(InstalledPackage::new(
                splits[0], splits[1], splits[2], splits[3],
            ));
        }

        if parsing_available {
            assert_eq!(3, splits.len());
            available.push(AvailablePackage::new(splits[0], splits[1], splits[2]));
        }
    }

    (installed, available)
}

fn load_list() -> String {
    let home = env::var("ANDROID_HOME").unwrap();

    let sdk_path = format!("{}/cmdline-tools/tools/bin/sdkmanager.bat", home);

    println!("sdk_path: {}", sdk_path);

    let command = Command::new(sdk_path)
        .arg("--list")
        .output()
        .expect("Error in attempt to get list of sdks");

    std::string::String::from_utf8(command.stdout).expect("Unable to convert output to utf8")
}
