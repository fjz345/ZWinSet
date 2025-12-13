use std::{
    fs::File,
    io::BufWriter,
    sync::{Arc, Mutex},
};

use crate::{
    all_jobs::ALL_JOBS,
    cli::CLI,
    image::{load_admin_icon, load_empty_icon},
    jobs::{Job, JobCategory, JobHandler},
    json_file::{JsonSelectedFiles, read_json_selected},
};

use eframe::{
    CreationContext,
    egui::{
        self, Color32, ImageSource, Layout, PointerButton, Pos2, ProgressBar, Response, RichText,
        ScrollArea, TextureHandle, Vec2,
    },
};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
enum AppState {
    #[default]
    Startup,
    UserSetup,
    UserEnsure,
    DoWork,
    AllWorkDone,
    Exit,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ZApp {
    monitor_size: Vec2,
    scale_factor: f32,
    native_pixel_per_point: f32,
    #[serde(skip)]
    state: AppState,
    #[serde(skip)]
    job_during_selection: Vec<(Job, bool)>,
    #[serde(skip)]
    job_handler: JobHandler,
    #[serde(skip)]
    log_buffer: Arc<Mutex<Vec<String>>>,
    #[serde(skip)]
    admin_icon: Option<TextureHandle>,
    #[serde(skip)]
    empty_icon: Option<TextureHandle>,
}

const HARDCODED_MONITOR_SIZE: Vec2 = Vec2::new(2560.0, 1440.0);
impl ZApp {
    // stupid work around since persistance storage does not work??
    pub fn request_init(&mut self) {
        self.state = AppState::Startup;
    }

    pub fn new(cc: &CreationContext<'_>, log_buffer: Arc<Mutex<Vec<String>>>) -> Self {
        // Can not get window screen size from CreationContext
        let monitor_size = HARDCODED_MONITOR_SIZE;
        const RESOLUTION_REF: f32 = 1080.0;
        let scale_factor: f32 = monitor_size.x.min(monitor_size.y) / RESOLUTION_REF;
        let native_pixel_per_point = cc.egui_ctx.native_pixels_per_point().unwrap_or(1.0);

        Self {
            monitor_size: monitor_size,
            scale_factor: scale_factor,
            native_pixel_per_point: native_pixel_per_point,
            state: AppState::Startup,
            log_buffer: log_buffer,
            ..Default::default()
        }
    }

    fn init_read_json_selected(&mut self) {
        let jobs_selected_from_file = read_json_selected("selected_jobs.json");
        match jobs_selected_from_file {
            Ok(r) => {
                let lowercase_r_result: Vec<_> =
                    r.selected_jobs.iter().map(|s| s.to_lowercase()).collect();
                self.job_during_selection
                    .iter_mut()
                    .for_each(|(job, active)| {
                        *active = lowercase_r_result.contains(&job.name().to_lowercase())
                    });
            }
            Err(e) => {
                log::error!("{e}");
                log::info!("Creating selected_jobs.json for next time...");
                // Create it for next time
                let file = File::create("selected_jobs.json");
                match file {
                    Ok(f) => {
                        let writer = BufWriter::new(f);
                        if let Err(e) =
                            serde_json::to_writer_pretty(writer, &JsonSelectedFiles::default())
                        {
                            log::error!("{e}");
                        }
                    }
                    Err(e) => log::error!("{e}"),
                }
            }
        }
    }

    fn init(&mut self) {
        self.job_during_selection = ALL_JOBS.iter().cloned().map(|job| (job, false)).collect();
        self.init_read_json_selected();
    }

    fn startup(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.admin_icon = Some(load_admin_icon(ctx));
        self.empty_icon = Some(load_empty_icon(ctx));
        self.init();

        let visuals: egui::Visuals = egui::Visuals::dark();
        ctx.set_visuals(visuals);
        log::info!("pixels_per_point: {:?}", ctx.pixels_per_point());
        log::info!(
            "native_pixels_per_point: {:?}",
            ctx.native_pixels_per_point()
        );
        ctx.set_pixels_per_point(self.scale_factor); // Maybe mult native_pixels_per_point?
        // ctx.set_debug_on_hover(true);

        // ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(true));
    }

    fn request_shutdown(&mut self) {
        self.state = AppState::Exit;
    }

    fn draw_ui_interactive_testing(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) -> Response {
        let outmost_response = egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
                ui.vertical(|ui| {
                    // Explination
                    ui.label("Select a job to execute");
                    ui.label("");

                    ui.vertical(|ui| {
                        for job_category in JobCategory::iter() {
                            let jobs_in_category: Vec<_> = ALL_JOBS
                                .iter()
                                .filter(|job| job.category() == job_category)
                                .collect();

                            if jobs_in_category.len() >= 1 {
                                ui.label(format!("{:?}", job_category));
                                ScrollArea::vertical()
                                    .id_salt(format!("scroll_area_testing_{:?}", job_category)) // corrected from `id_salt` to `id_source`
                                    .max_height(f32::INFINITY)
                                    .show(ui, |ui| {
                                        ui.vertical(|ui| {
                                            for job in jobs_in_category {
                                                let job_name = format!("{}", job.name());
                                                ui.horizontal(|ui| {
                                                    let icon_id = if job.require_admin() {
                                                        self.admin_icon.clone().unwrap().id()
                                                    } else {
                                                        self.empty_icon.clone().unwrap().id()
                                                    };
                                                    ui.image(ImageSource::Texture(
                                                        egui::load::SizedTexture {
                                                            id: icon_id,
                                                            size: [12.0, 15.0].into(),
                                                        },
                                                    ));
                                                    if ui.button(job_name).clicked() {
                                                        self.job_handler
                                                            .set_jobs(vec![job.clone()]);
                                                        self.state = AppState::DoWork;
                                                    }
                                                });
                                            }
                                        });
                                    });
                            }
                        }
                    });
                });
            })
        });

        outmost_response.response
    }

    fn draw_checkbox_job(
        ui: &mut egui::Ui,
        job: &Job,
        admin_icon: TextureHandle,
        empty_icon: TextureHandle,
        job_during_selection: &mut Vec<(Job, bool)>,
    ) -> (Response, bool) {
        let job_name = format!("{}", job.name());
        let icon_id = if job.require_admin() {
            admin_icon.id()
        } else {
            empty_icon.id()
        };
        let mut is_checked = false;
        let response = ui.horizontal(|ui| {
            ui.image(ImageSource::Texture(egui::load::SizedTexture {
                id: icon_id,
                size: [12.0, 15.0].into(),
            }));
            let mut checkbox_value = job_during_selection
                .iter_mut()
                .find(|(jjob, _value)| jjob == job)
                .map(|f| &mut f.1)
                .expect("failure");

            let job_text_color = match job.ready_state() {
                crate::jobs::JobReadyState::NOTTESTED => Color32::YELLOW,
                crate::jobs::JobReadyState::VERIFIED => Color32::WHITE,
                crate::jobs::JobReadyState::NOTWORKING
                | crate::jobs::JobReadyState::NOTIMPLEMENTED => Color32::RED,
            };
            let job_text = RichText::new(&job_name).color(job_text_color);
            let checkbox_response = ui.checkbox(&mut checkbox_value, job_text);
            is_checked = *checkbox_value;
            checkbox_response
        });
        (response.inner, is_checked)
    }

    fn draw_ui_usersetup(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) -> Response {
        let outmost_response = egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
                let mut job_check_responses: Vec<(Response, bool, &Job)> = Vec::new();

                ui.vertical(|ui| {
                    // Explination
                    ui.label("Select desiered jobs, then press next.");
                    ui.label(RichText::new("White text is verified working").color(Color32::WHITE));
                    ui.label(
                        RichText::new("Yellow text is not tested (probably works)")
                            .color(Color32::YELLOW),
                    );
                    ui.label(RichText::new("Red text is not working").color(Color32::RED));
                    ui.label("");
                    ui.horizontal(|ui| {
                        for job_category in JobCategory::iter() {
                            let jobs_in_category: Vec<_> = ALL_JOBS
                                .iter()
                                .filter(|job| job.category() == job_category)
                                .collect();

                            if jobs_in_category.len() >= 1 {
                                ui.vertical(|ui| {
                                    ui.label(format!("{:?}", job_category));
                                    ui.horizontal(|ui| {
                                        ScrollArea::vertical()
                                            .id_salt(format!("scroll_area_{:?}", job_category)) // corrected from `id_salt` to `id_source`
                                            .min_scrolled_height(400.0)
                                            .show(ui, |ui| {
                                                ui.vertical(|ui| {
                                                    for job in jobs_in_category {
                                                        let (checkbox_response, checkbox_value) =
                                                            Self::draw_checkbox_job(
                                                                ui,
                                                                job,
                                                                self.admin_icon.clone().unwrap(),
                                                                self.empty_icon.clone().unwrap(),
                                                                &mut self.job_during_selection,
                                                            );
                                                        job_check_responses.push((
                                                            checkbox_response,
                                                            checkbox_value,
                                                            job,
                                                        ));
                                                    }
                                                });
                                            });
                                    });
                                });
                            }
                        }
                    });

                    ui.label("");
                    ui.horizontal(|ui| {
                        // Continue button
                        if ui.button("Next").clicked() {
                            // Collect jobs selected
                            let checked_jobs: Vec<_> = job_check_responses
                                .iter()
                                .filter(|response| response.1)
                                .map(|a| a.2.clone())
                                .collect();

                            self.job_handler.set_jobs(checked_jobs);
                            self.state = AppState::UserEnsure;
                        }
                    });

                    ui.label("Tested on Windows 10");
                    ui.label("Made by FjZ345");
                });
            })
        });

        outmost_response.inner.response
    }

    fn draw_ui_userensure(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) -> Response {
        let outmost_response = egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
                let mut job_check_responses: Vec<(Response, bool, Job)> = Vec::new();

                ui.vertical(|ui| {
                    ui.label("Are you sure you would like to execute these jobs?");
                    ui.label("");
                    ui.label("Selected Jobs:");
                    ScrollArea::vertical()
                        .id_salt("scroll_area_userensure") // corrected from `id_salt` to `id_source`
                        .max_height(f32::INFINITY)
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                for job in self.job_handler.get_jobs() {
                                    ui.horizontal(|ui| {
                                        let job_clone = job.clone();
                                        let (checkbox_response, checkbox_value) =
                                            Self::draw_checkbox_job(
                                                ui,
                                                &job_clone,
                                                self.admin_icon.clone().unwrap(),
                                                self.empty_icon.clone().unwrap(),
                                                &mut self.job_during_selection,
                                            );
                                        job_check_responses.push((
                                            checkbox_response,
                                            checkbox_value,
                                            job_clone,
                                        ));
                                    });
                                }
                            });
                        });

                    ui.label("");
                    ui.horizontal(|ui| {
                        if ui.button("Back").clicked() {
                            self.state = AppState::UserSetup;
                        }
                        if ui.button("Next").clicked() {
                            let checked_jobs: Vec<_> = job_check_responses
                                .iter()
                                .filter(|response| response.1)
                                .map(|a| a.2.clone())
                                .collect();

                            self.job_handler.set_jobs(checked_jobs);
                            self.state = AppState::DoWork;
                        }

                        // Collect jobs selected
                    });

                    ui.label("Tested on Windows 10");
                    ui.label("Made by FjZ345");
                });
            })
        });

        outmost_response.inner.response
    }

    fn draw_ui_dowork(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) -> Response {
        let outmost_response = egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
                ui.vertical(|ui| {
                    ui.label("Executing Jobs...");
                    ScrollArea::vertical()
                        .id_salt("scroll_area_dowork") // corrected from `id_salt` to `id_source`
                        .max_height(f32::INFINITY)
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                let job_progress = self.job_handler.get_job_progress();
                                for (job, job_status) in job_progress {
                                    let job_name = format!("{}", job.name());
                                    ui.horizontal(|ui| {
                                        let icon_id = if job.require_admin() {
                                            self.admin_icon.clone().unwrap().id()
                                        } else {
                                            self.empty_icon.clone().unwrap().id()
                                        };
                                        ui.image(ImageSource::Texture(egui::load::SizedTexture {
                                            id: icon_id,
                                            size: [12.0, 15.0].into(),
                                        }));
                                        let progress_bar = ProgressBar::new(match job_status {
                                            crate::jobs::JobStatus::NotStarted => 0.0,
                                            crate::jobs::JobStatus::InProgress(p) => p,
                                            crate::jobs::JobStatus::Failed(p) => p,
                                            crate::jobs::JobStatus::Finished => 1.0,
                                        })
                                        .show_percentage()
                                        .desired_width(100.0);
                                        ui.add(progress_bar);
                                        ui.label(job_name);
                                    });
                                }
                            });
                        });

                    ui.label("");
                    if self.job_handler.finished() {
                        self.state = AppState::AllWorkDone;
                    }
                });
            })
        });

        outmost_response.inner.response
    }

    fn draw_ui_finished(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) -> Response {
        let outmost_response = egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
                ui.vertical(|ui| {
                    ui.label("Finished executing jobs...");
                    ScrollArea::vertical()
                        .id_salt("scroll_area_dowork") // corrected from `id_salt` to `id_source`
                        .max_height(f32::INFINITY)
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                let job_progress = self.job_handler.get_job_progress();
                                for (job, job_status) in job_progress {
                                    let job_name = format!("{}", job.name());
                                    ui.horizontal(|ui| {
                                        let icon_id = if job.require_admin() {
                                            self.admin_icon.clone().unwrap().id()
                                        } else {
                                            self.empty_icon.clone().unwrap().id()
                                        };
                                        ui.image(ImageSource::Texture(egui::load::SizedTexture {
                                            id: icon_id,
                                            size: [12.0, 15.0].into(),
                                        }));
                                        let progress_bar = ProgressBar::new(match job_status {
                                            crate::jobs::JobStatus::NotStarted => 0.0,
                                            crate::jobs::JobStatus::InProgress(p) => p,
                                            crate::jobs::JobStatus::Failed(p) => p,
                                            crate::jobs::JobStatus::Finished => 1.0,
                                        })
                                        .show_percentage()
                                        .desired_width(100.0);
                                        ui.add(progress_bar);
                                        ui.label(job_name);
                                    });
                                }
                            });
                        });

                    ui.label("");
                    ui.horizontal(|ui| {
                        if ui.button("To Start").clicked() {
                            if self.job_handler.finished() {
                                self.state = AppState::Startup;
                            }
                        }
                        if ui.button("Exit").clicked() {
                            if self.job_handler.finished() {
                                self.state = AppState::Exit;
                            }
                        }
                    });
                });
            })
        });

        outmost_response.inner.response
    }

    fn process_ctx_inputs(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut user_quit: bool = false;
        {
            let _input_ctx = ctx.input(|r| {
                // Esc
                if r.key_down(egui::Key::Escape) {
                    user_quit = true;
                }

                // DoubleLeftClick
                if r.pointer.button_double_clicked(PointerButton::Primary) {
                    let mouse_pos = r.pointer.interact_pos().unwrap();
                    log::info!("double click @({},{})", mouse_pos.x, mouse_pos.y);
                }

                if r.pointer.button_clicked(PointerButton::Middle) {
                    let mouse_pos: Pos2 = r.pointer.interact_pos().unwrap();

                    log::info!("middle click @({},{})", mouse_pos.x, mouse_pos.y);
                }
            });
        }

        if user_quit {
            self.request_shutdown();
        }
    }
}

const WINDOWS_WINDOW_BAR_HEIGHT: Vec2 = Vec2::new(0.0, 16.0);
impl eframe::App for ZApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        log::info!("SAVING...");

        #[cfg(feature = "serde")]
        if let Ok(json) = serde_json::to_string(self) {
            log::debug!("SAVED with state: {:?}", self.state);
            storage.set_string(eframe::APP_KEY, json);
        }
        log::info!("SAVED!");
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let interactive_testing = CLI
            .get()
            .map(|c| c.interactive_mode.is_some())
            .unwrap_or(false);

        match self.state {
            AppState::Startup => {
                self.startup(ctx, frame);
                self.state = AppState::UserSetup;
            }
            AppState::UserSetup => {
                let response = if interactive_testing {
                    self.draw_ui_interactive_testing(ctx, frame)
                } else {
                    self.draw_ui_usersetup(ctx, frame)
                };
                ctx.send_viewport_cmd(egui::ViewportCommand::MinInnerSize(
                    response.rect.size() + WINDOWS_WINDOW_BAR_HEIGHT,
                ));
            }
            AppState::UserEnsure => {
                let response = self.draw_ui_userensure(ctx, frame);
                ctx.send_viewport_cmd(egui::ViewportCommand::MinInnerSize(
                    response.rect.size() + WINDOWS_WINDOW_BAR_HEIGHT,
                ));
            }
            AppState::DoWork => {
                let response = self.draw_ui_dowork(ctx, frame);
                self.job_handler.update();
                ctx.request_repaint();
                ctx.send_viewport_cmd(egui::ViewportCommand::MinInnerSize(
                    response.rect.size() + WINDOWS_WINDOW_BAR_HEIGHT,
                ));
            }
            AppState::AllWorkDone => {
                if interactive_testing {
                    self.state = AppState::UserSetup;
                    return;
                }
                let response = self.draw_ui_finished(ctx, frame);
                ctx.send_viewport_cmd(egui::ViewportCommand::MinInnerSize(
                    response.rect.size() + WINDOWS_WINDOW_BAR_HEIGHT,
                ));
            }
            AppState::Exit => {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            _ => {
                panic!("Not a valid state {:?}", self.state);
            }
        }
    }
}
