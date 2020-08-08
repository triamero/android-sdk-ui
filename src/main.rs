use std::env;
use std::sync::Arc;
use std::thread;

use druid::lens::{self, LensExt};
use druid::widget::{Button, CrossAxisAlignment, Flex, Label, List, Scroll};
use druid::{
    AppLauncher, Color, Data, Lens, LocalizedString, UnitPoint, Widget, WidgetExt, WindowDesc,
};
mod sdk;

#[derive(Clone, Data, Lens)]
struct AppData {
    items: Arc<Vec<u32>>,
}

fn main() {
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

fn load_data() {
    let packages = sdk::list_packages();
}
