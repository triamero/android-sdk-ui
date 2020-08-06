use std::env;
use std::sync::Arc;
use std::thread;
use std::error::Error;

use druid::lens::{self, LensExt};
use druid::widget::{Button, CrossAxisAlignment, Flex, Label, List, Scroll};
use druid::{
    AppLauncher, Color, Data, Lens, LocalizedString, UnitPoint, Widget, WidgetExt, WindowDesc,
};

use data::{InstalledPackage, Package, Version};
mod data;

#[derive(Clone, Data, Lens)]
struct AppData {
    items: Arc<Vec<u32>>,
}

fn main() {
    let output = load_data();

    print!("{}", output);

    let mut installed_packages = false;
    let mut available_packages = false;
    let mut available_updates = false;

    let mut header_skipped = false;
    let mut table_head_skipped = false;

    let mut installed: Vec<InstalledPackage> = Vec::new();

    let lines: Vec<&str> = output.lines().collect();

    //println!("{}", lines);

    for line in lines {
        //println!("{}", line);

        if line.starts_with("Installed packages") {
            println!("Found installed packages line");
            installed_packages = true;
            available_packages = false;
            available_updates = false;

            header_skipped = false;
            table_head_skipped = false;
            continue;
        } else if line.starts_with("Available Packages:") {
            installed_packages = false;
            available_packages = true;
            available_updates = false;

            header_skipped = false;
            table_head_skipped = false;
            continue;
        } else if line.starts_with("Available Updates:") {
            installed_packages = false;
            available_packages = false;
            available_updates = true;

            header_skipped = false;
            table_head_skipped = false;
            continue;
        }

        // пропуск заголовков таблицы
        if installed_packages || available_packages || available_updates {
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



        if installed_packages {
            let splits: Vec<&str> = line.split("|").collect();

            let name = splits[0];
            let version = splits[1];
            let description = splits[2];
            let location = splits[3];

            installed.push(InstalledPackage::new(
                splits[0], splits[1], splits[2], splits[3],
            ));
        }
    }

    println!("installed packages: {}", installed.len());

    for package in installed {
        println!("{} {} {} {}", package.path, package.version, package.description, package.location)
    }
}

fn main1() {
    let main_window = WindowDesc::new(ui_builder)
        .title(LocalizedString::new("list-demo-window-title").with_placeholder("Android SDK"));
    // Set our initial data
    let data = AppData {
        items: Arc::new(vec![1, 2, 3]),
    };
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("launch failed");
}

fn ui_builder() -> impl Widget<AppData> {
    let mut root = Flex::column();

    root.add_child(Label::new(env::var("ANDROID_HOME").unwrap()));

    // Build a button to add children to both lists
    root.add_child(
        Button::new("Add")
            .on_click(|_, data: &mut AppData, _| {
                // Add child to right list
                let value = data.items.len() + 1;
                Arc::make_mut(&mut data.items).push(value as u32);
            })
            .fix_height(30.0)
            .expand_width(),
    );

    let mut lists = Flex::row().cross_axis_alignment(CrossAxisAlignment::Start);

    // Build a list with shared data
    lists.add_flex_child(
        Scroll::new(List::new(|| {
            Flex::row()
                .with_child(
                    Label::new(|(_, item): &(Arc<Vec<u32>>, u32), _env: &_| {
                        format!("List item #{}", item)
                    })
                    .align_vertical(UnitPoint::LEFT),
                )
                .with_flex_spacer(1.0)
                .with_child(
                    Button::new("Delete")
                        .on_click(|_ctx, (shared, item): &mut (Arc<Vec<u32>>, u32), _env| {
                            // We have access to both child's data and shared data.
                            // Remove element from right list.
                            Arc::make_mut(shared).retain(|v| v != item);
                        })
                        .fix_size(80.0, 20.0)
                        .align_vertical(UnitPoint::CENTER),
                )
                .padding(10.0)
                .background(Color::rgb(0.5, 0.0, 0.5))
                .fix_height(50.0)
        }))
        .vertical()
        .lens(lens::Id.map(
            // Expose shared data with children data
            |d: &AppData| (d.items.clone(), d.items.clone()),
            |d: &mut AppData, x: (Arc<Vec<u32>>, Arc<Vec<u32>>)| {
                // If shared data was changed reflect the changes in our AppData
                d.items = x.0
            },
        )),
        1.0,
    );

    root.add_flex_child(lists, 1.0);

    let t = thread::spawn(load_data);

    t.join().unwrap();

    // Mark the widget as needing its layout rects painted
    return root.debug_paint_layout();
}

fn load_data() -> String {
    let home = env::var("ANDROID_HOME").unwrap();

    let sdk_path = format!("{}/cmdline-tools/tools/bin/sdkmanager.bat", home);

    println!("sdk_path: {}", sdk_path);

    let process = std::process::Command::new(sdk_path)
        .arg("--list")
        .output()
        .expect("Error in attempt to get list of sdks");

    std::string::String::from_utf8(process.stdout).expect("Unable to convert output to utf8")
}
