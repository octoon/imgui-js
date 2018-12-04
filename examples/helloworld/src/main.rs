#[macro_use]
extern crate stdweb;

extern crate imgui_rust;

fn main() {
    stdweb::initialize();

    let im = imgui_rust::Imgui::new();
    im.IMGUI_CHECKVERSION();
}
