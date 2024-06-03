use std::cmp::min;

use egui::{Color32, Context, Sense};

use crate::{
    run::{category::Category, game::Game},
    HitSplit,
};

pub fn counter(app: &mut HitSplit, ctx: &Context) {
    let counter_clicked = egui::CentralPanel::default()
        .show(ctx, |ui| {
            ui.style_mut()
                .text_styles
                .get_mut(&egui::TextStyle::Body)
                .unwrap()
                .size = app.config.font_size;
            ui.vertical_centered(|ui| {
                ui.label(match &app.loaded_game {
                    Some(game) => game.name.clone(),
                    None => Game::default().name,
                });
            });
            if app.loaded_category.is_some() {
                ui.vertical_centered(|ui| {
                    ui.label(match &app.loaded_category {
                        Some(category) => category.name.clone(),
                        None => Category::default().name,
                    });
                });
            }

            ui.vertical(|ui| {
                let table = egui_extras::TableBuilder::new(ui)
                    .striped(false)
                    .cell_layout(egui::Layout::centered_and_justified(
                        egui::Direction::LeftToRight,
                    ))
                    .resizable(true)
                    .striped(false)
                    .column(egui_extras::Column::exact(app.config.font_size))
                    .column(egui_extras::Column::auto())
                    .column(egui_extras::Column::auto())
                    .column(egui_extras::Column::auto())
                    .column(egui_extras::Column::auto())
                    .min_scrolled_height(200.0);
                let mut color = Color32::from_rgb(250, 250, 250);
                if !app.config.dark_mode {
                    color = Color32::from_rgb(8, 8, 8)
                }
                let binding = Vec::new();
                let splits = match &app.loaded_category {
                    Some(category) => &category.splits,
                    None => &binding,
                };
                let first_split: usize = app.selected_split
                    - min(app.config.num_splits_counter >> 1, app.selected_split);
                let last_split: usize =
                    min(first_split + app.config.num_splits_counter, splits.len());
                let first_split = min(
                    first_split,
                    last_split
                        .checked_sub(app.config.num_splits_counter)
                        .unwrap_or_default(),
                );
                table
                    .header(app.config.font_size + 5.0, |mut header| {
                        header.col(|_| {});
                        header.col(|ui| {
                            ui.strong("Name");
                        });
                        header.col(|ui| {
                            ui.strong("Hits");
                        });
                        header.col(|ui| {
                            ui.strong("Diff");
                        });
                        header.col(|ui| {
                            ui.strong("PB");
                        });
                    })
                    .body(|mut body| {
                        splits
                            .iter()
                            .enumerate()
                            .filter(|(i, _)| {
                                !app.config.limit_splits_shown
                                    || (i >= &first_split && i < &last_split)
                            })
                            .for_each(|(i, split)| {
                                let mut label_color = color;
                                if i <= app.selected_split {
                                    if split.hits == 0 {
                                        label_color = Color32::from_rgb(8, 250, 8);
                                    } else if split.hits < split.pb {
                                        label_color = Color32::from_rgb(250, 250, 8);
                                    } else {
                                        label_color = Color32::from_rgb(250, 8, 8);
                                    }
                                }
                                body.row(app.config.font_size + 5.0, |mut row| {
                                    let mut name = split.name.clone();
                                    if i == app.selected_split {
                                        name = format!("> {}", name);
                                    }
                                    row.col(|ui| {
                                        if let Some(p) = &split.icon_path {
                                            let path = p.as_path().to_str().unwrap();
                                            ui.add(
                                                egui::Image::new(format!("file://{path}"))
                                                    .max_height(app.config.font_size),
                                            );
                                        }
                                    });
                                    row.col(|ui| {
                                        ui.colored_label(label_color, name);
                                    });
                                    row.col(|ui| {
                                        ui.colored_label(label_color, split.hits.to_string());
                                    });
                                    row.col(|ui| {
                                        ui.colored_label(
                                            label_color,
                                            (i32::from(split.hits) - i32::from(split.pb))
                                                .to_string(),
                                        );
                                    });
                                    row.col(|ui| {
                                        ui.colored_label(label_color, split.pb.to_string());
                                    });
                                });
                            });
                        body.row(app.config.font_size + 5.0, |mut row| {
                            row.col(|_| {});
                            row.col(|ui| {
                                ui.colored_label(color, "Total: ");
                            });
                            row.col(|ui| {
                                let hits = match &app.loaded_category {
                                    Some(category) => {
                                        category.splits.iter().map(|split| split.hits).sum::<u16>()
                                    }
                                    None => 0,
                                };

                                ui.colored_label(color, hits.to_string());
                            });
                            row.col(|ui| {
                                let diff = match &app.loaded_category {
                                    Some(category) => category
                                        .splits
                                        .iter()
                                        .map(|split| i32::from(split.hits) - i32::from(split.pb))
                                        .sum::<i32>(),
                                    None => 0,
                                };
                                ui.colored_label(color, diff.to_string());
                            });
                            row.col(|ui| {
                                let pb = match &app.loaded_category {
                                    Some(category) => {
                                        category.splits.iter().map(|split| split.pb).sum::<u16>()
                                    }
                                    None => 0,
                                };
                                ui.colored_label(color, pb.to_string());
                            });
                        });
                    });
            });
        })
        .response
        .interact(Sense::click())
        .secondary_clicked();
    if counter_clicked {
        app.show_config = true;
    };
}
