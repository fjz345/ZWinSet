use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

use crate::{
    all_jobs::ALL_JOBS,
    error::Result,
    jobs::{Job, JobCategory, JobHandler, PowerShellCtx},
};

use eframe::{
    CreationContext,
    egui::{
        self, Layout, PointerButton, Pos2, ProgressBar, Response, ScrollArea, Vec2, WidgetText,
    },
};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::{commands::test_cmd, logger::LogCollector};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
enum AppState {
    #[default]
    Startup,
    UserSetup,
    UserEnsure,
    DoWork,
    AllWorkDone,
    Exit,
    InteractiveTesting,
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
}

const HARDCODED_MONITOR_SIZE: Vec2 = Vec2::new(2560.0, 1440.0);
impl ZApp {
    const INTERACTIVE_TESTING: bool = true;
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

    fn init(&mut self) {
        self.job_during_selection = ALL_JOBS.iter().cloned().map(|job| (job, false)).collect();
    }

    fn startup(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.init();

        let visuals: egui::Visuals = egui::Visuals::dark();
        ctx.set_visuals(visuals);
        log::info!("pixels_per_point{:?}", ctx.pixels_per_point());
        log::info!("native_pixels_per_point{:?}", ctx.native_pixels_per_point());
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
            ui.with_layout(Layout::left_to_right(egui::Align::Min), |mut ui| {
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
                                    .max_height(400.0)
                                    .show(ui, |ui| {
                                        ui.vertical(|ui| {
                                            for job in jobs_in_category {
                                                let job_name = format!("{}", job.name());
                                                ui.horizontal(|ui| {
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
    fn draw_ui_usersetup(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) -> Response {
        let outmost_response = egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::left_to_right(egui::Align::Min), |mut ui| {
                let mut job_check_responses: Vec<(Response, bool, &Job)> = Vec::new();

                ui.vertical(|ui| {
                    // Explination
                    ui.label("Select desiered jobs, then press next to do the jobs");
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
                                    .id_salt(format!("scroll_area_{:?}", job_category)) // corrected from `id_salt` to `id_source`
                                    .max_height(400.0)
                                    .show(ui, |ui| {
                                        ui.vertical(|ui| {
                                            for job in jobs_in_category {
                                                let job_name = format!("{}", job.name());
                                                ui.horizontal(|ui| {
                                                    let mut checkbox_value = self
                                                        .job_during_selection
                                                        .iter_mut()
                                                        .find(|(jjob, value)| jjob == job)
                                                        .map(|f| &mut f.1)
                                                        .expect("failure");
                                                    let checkbox_response =
                                                        ui.checkbox(&mut checkbox_value, &job_name);
                                                    job_check_responses.push((
                                                        checkbox_response,
                                                        *checkbox_value,
                                                        job,
                                                    ));
                                                });
                                            }
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

                    // Made by FjZ345
                    ui.label("Made by FjZ345");
                });
            })
        });

        outmost_response.response
    }

    fn draw_ui_userensure(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) -> Response {
        let outmost_response = egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::left_to_right(egui::Align::Min), |mut ui| {
                ui.vertical(|ui| {
                    ui.label("Are you sure you would like to execute these jobs?");
                    ui.label("");
                    ui.label("Selected Jobs:");
                    ScrollArea::vertical()
                        .id_salt("scroll_area_userensure") // corrected from `id_salt` to `id_source`
                        .max_height(400.0)
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                for job in self.job_handler.get_jobs() {
                                    let job_name = format!("{}", job.name());
                                    ui.horizontal(|ui| {
                                        ui.checkbox(&mut true, &job_name);
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
                            self.state = AppState::DoWork;
                        }
                    });
                });
            })
        });

        outmost_response.response
    }

    fn draw_ui_dowork(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) -> Response {
        let outmost_response = egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::left_to_right(egui::Align::Min), |mut ui| {
                ui.vertical(|ui| {
                    ui.label("Executing Jobs...");
                    ScrollArea::vertical()
                        .id_salt("scroll_area_dowork") // corrected from `id_salt` to `id_source`
                        .max_height(400.0)
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                let job_progress = self.job_handler.get_job_progress();
                                for (job, progress) in job_progress {
                                    let job_name = format!("{}", job.name());
                                    ui.horizontal(|ui| {
                                        let progress_bar = ProgressBar::new(progress)
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
                        if self.job_handler.finished() {
                            self.state = AppState::AllWorkDone;
                        }
                    });
                });
            })
        });

        outmost_response.response
    }

    fn draw_ui_finished(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) -> Response {
        let outmost_response = egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::left_to_right(egui::Align::Min), |mut ui| {
                ui.vertical(|ui| {
                    ui.label("Finished executing jobs...");
                    ScrollArea::vertical()
                        .id_salt("scroll_area_dowork") // corrected from `id_salt` to `id_source`
                        .max_height(400.0)
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                let job_progress = self.job_handler.get_job_progress();
                                for (job, progress) in job_progress {
                                    let job_name = format!("{}", job.name());
                                    ui.horizontal(|ui| {
                                        let progress_bar = ProgressBar::new(progress)
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
                        if ui.button("Exit").clicked() {
                            if self.job_handler.finished() {
                                self.state = AppState::Exit;
                            }
                        }
                    });
                });
            })
        });

        outmost_response.response
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
        match self.state {
            AppState::Startup => {
                self.startup(ctx, frame);
                self.state = AppState::UserSetup;
            }
            AppState::InteractiveTesting => {}
            AppState::UserSetup => {
                if Self::INTERACTIVE_TESTING {
                    self.draw_ui_interactive_testing(ctx, frame);
                } else {
                    self.draw_ui_usersetup(ctx, frame);
                }
            }
            AppState::UserEnsure => {
                self.draw_ui_userensure(ctx, frame);
            }
            AppState::DoWork => {
                self.draw_ui_dowork(ctx, frame);
                self.job_handler.update();
            }
            AppState::AllWorkDone => {
                if Self::INTERACTIVE_TESTING {
                    self.state = AppState::UserSetup;
                    return;
                }
                self.draw_ui_finished(ctx, frame);
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
