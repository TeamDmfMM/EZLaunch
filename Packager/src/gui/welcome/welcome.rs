extern crate conrod;

extern crate find_folder;
extern crate piston_window;

use conrod::{Canvas, Colorable, Image, Positionable, Sizeable, Widget, color};
use piston_window::{EventLoop, Flip, ImageSize, G2dTexture, PistonWindow, Texture, UpdateEvent, TextureSettings};
use piston_window::Window;
use piston_window::AdvancedWindow;
use welcome::consts;

pub fn show_splash(length: i32) {
    let mut window: PistonWindow =
    piston_window::WindowSettings::new("EZLaunch Creator",
                                       [consts::WIDTH, consts::HEIGHT]).opengl(piston_window::OpenGL::V3_2)
        .exit_on_esc(false).vsync(true).samples(4).build().unwrap();

    window.set_ups(60);

    widget_ids! (
        BACKGROUND_CANVAS,
        TEXT_GROUPER,
        HEADER_TEXT,
        VERSION_TEXT,
        NEW_BUTTON,
        OPEN_BUTTON
    );

    let mut ui: conrod::Ui = conrod::UiBuilder::new().build();

    let assets_folder = find_folder::Search::KidsThenParents(5, 5).for_folder("assets").unwrap();
    let font_path = assets_folder.join("font/OpenSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();

    let mut text_texture_cache = conrod::backend::piston_window::GlyphCache::new(&mut window, consts::WIDTH, consts::HEIGHT);


    window.set_position((200, 65));
    while let Some(event) = window.next() {

        ui.handle_event(event.clone());

        window.draw_2d(&event, |c, g| {
            if let Some(primitives) = ui.draw_if_changed() {
                fn texture_from_image<T>(img: &T) -> &T { img };
                conrod::backend::piston_window::draw(c, g, primitives,
                                                     &mut text_texture_cache,
                                                     &image_map,
                                                     texture_from_image);
            }
        });

        event.update(|_| ui.set_widgets(|mut ui| {
            Canvas::new().color(color::CHARCOAL).flow_down(&[
                (TEXT_GROUPER, Canvas::new()),
                (NEW_BUTTON, )
            ])
        }));
    }

}