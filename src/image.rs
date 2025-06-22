use eframe::egui::{self, ColorImage, TextureHandle};

const ADMIN_ICON: &[u8] = include_bytes!("../img/admin_icon.png");
const EMPTY_ICON: &[u8] = include_bytes!("../img/empty.png");

pub fn load_admin_icon(ctx: &egui::Context) -> TextureHandle {
    // Load image from bytes (using the `image` crate)
    let img = image::load_from_memory(ADMIN_ICON).expect("Failed to load image");
    let size = [img.width() as usize, img.height() as usize];
    let img_rgba = img.to_rgba8();
    let pixels: Vec<egui::Color32> = img_rgba
        .pixels()
        .map(|p| egui::Color32::from_rgba_premultiplied(p[0], p[1], p[2], p[3]))
        .collect();

    let color_image = ColorImage { size, pixels };

    ctx.load_texture("admin_icon", color_image, egui::TextureOptions::default())
}

pub fn load_empty_icon(ctx: &egui::Context) -> TextureHandle {
    // Load image from bytes (using the `image` crate)
    let img = image::load_from_memory(EMPTY_ICON).expect("Failed to load image");
    let size = [img.width() as usize, img.height() as usize];
    let img_rgba = img.to_rgba8();
    let pixels: Vec<egui::Color32> = img_rgba
        .pixels()
        .map(|p| egui::Color32::from_rgba_premultiplied(p[0], p[1], p[2], p[3]))
        .collect();

    let color_image = ColorImage { size, pixels };

    ctx.load_texture("admin_icon", color_image, egui::TextureOptions::default())
}
