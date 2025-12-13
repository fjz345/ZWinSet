// #![windows_subsystem = "windows"]
#![allow(dead_code)]
#![allow(unreachable_patterns)]

use clap::Parser;
use eframe::egui::{self};
use std::{env, sync::OnceLock};

use crate::{app::ZApp, logger::LogCollector};

mod all_jobs;
mod app;
mod commands;
mod error;
mod image;
mod jobs;
mod json_file;
mod logger;
mod threadsafe_atomic_counter;
mod windows;

#[derive(Debug, Parser)]
#[command(name = "ZWinSet")]
#[command(author, version, about = "App with CLI support")]
struct Cli {
    #[arg(long)]
    debug: bool,

    #[arg(short, long)]
    interactive_mode: Option<String>,

    #[arg(short, long)]
    config: Option<String>,

    #[arg(last = true)]
    extra: Vec<String>,
}
static CLI: OnceLock<Cli> = OnceLock::new();

fn main() -> eframe::Result {
    let cli = Cli::parse();
    CLI.set(cli).expect("CLI already set");

    unsafe { env::set_var("RUST_LOG", "debug") }; // or "info" or "debug"
    let log_buffer = LogCollector::init().expect("Failed to init logger");

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([2560.0, 1440.0]),
        ..Default::default()
    };

    eframe::run_native(
        "ZWinSet",
        native_options,
        Box::new(move |cc: &eframe::CreationContext<'_>| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            #[cfg(feature = "serde")]
            {
                // Try to load saved state from storage
                if let Some(storage) = cc.storage {
                    if let Some(json) = storage.get_string(eframe::APP_KEY) {
                        if let Ok(mut app) = serde_json::from_str::<ZApp>(&json) {
                            log::info!("Found previous app storage");
                            app.request_init();
                            return Ok(Box::new(app));
                        }
                    }
                }
            }

            let app = ZApp::new(cc, log_buffer.clone());
            Ok(Box::<ZApp>::new(app))
        }),
    )
}
