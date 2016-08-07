//! Displays a quick splash screen
//!
//! mm12 was here
extern crate conrod;

extern crate find_folder;
extern crate piston_window;

use conrod::{Canvas, Colorable, Image, Positionable, Sizeable, Widget, color};
use piston_window::{EventLoop, Flip, ImageSize, G2dTexture, PistonWindow, Texture, UpdateEvent, TextureSettings};
use piston_window::Window;
use piston_window::AdvancedWindow;
use splasher::consts;

pub fn show_splash() {
    let thing_is_a_ctime_test = consts::WIDTH;

    let mut window: PistonWindow =
        piston_window::WindowSettings::new("EZLaunch Creator",
            [300 as u32, 512 as u32]).opengl(piston_window::OpenGL::V3_2)
            .exit_on_esc(false).vsync(true).samples(4).decorated(false).build().unwrap();

    window.set_ups(60);

    widget_ids! (
        BACKDROP,
        SPLASH
    );

    let mut ui: conrod::Ui = conrod::UiBuilder::new().build();

    let mut text_texture_cache = conrod::backend::piston_window::GlyphCache::new(&mut window, 0, 0);

    let image_map = image_map! {
        (SPLASH, get_splash_image(&mut window))
    };

    let w: i32 = 300;
    let h: i32 = 512;



    let mut counter = 0;
    window.set_position((200, 65));
    while let Some(event) = window.next() {
        counter += 1;

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
            Canvas::new().color(color::Color::Rgba(0.996078, 0.6627450, 0.06666, 1.0)).set(BACKDROP, &mut ui);

            Image::new().w_h(w as f64, h as f64).middle().set(SPLASH, &mut ui);
        }));

        if counter > 500 {
            window.set_should_close(true);
        }

    }

}

fn get_splash_image(window: &mut PistonWindow) -> G2dTexture<'static> {
    let assets_folder = find_folder::Search::KidsThenParents(5, 5).for_folder("assets").unwrap();
    let splash_image_path = assets_folder.join("splash.png");

    let factory = &mut window.factory;
    let texture_settings = TextureSettings::new();
    Texture::from_path(factory, &splash_image_path, Flip::None, &texture_settings).unwrap()
}