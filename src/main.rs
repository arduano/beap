mod beap;

#[cfg(test)]
mod tests;

use std::time::{Duration, Instant};

use beap::{
    animation_util::{AnimatedSwap, StepTracker},
    Beap, BeapCoordinate,
};
use eframe::{
    egui::{self, Sense, Slider},
    emath::Align2,
    epaint::{FontId, Pos2},
};

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct SwapAnimationSequence {
    step: AnimatedSwap,
    iter: Box<dyn Iterator<Item = AnimatedSwap>>,
    start: Instant,
    end: Instant,
}

impl SwapAnimationSequence {
    fn new(
        mut iter: Box<dyn Iterator<Item = AnimatedSwap>>,
        step_duration: Duration,
    ) -> Option<SwapAnimationSequence> {
        let start = Instant::now();
        let end = start + step_duration;
        let step = iter.next()?;
        Some(Self {
            step,
            iter,
            start,
            end,
        })
    }

    fn next(mut self, step_duration: Duration) -> Option<SwapAnimationSequence> {
        let start = Instant::now();
        let end = start + step_duration;
        let step = self.iter.next()?;
        Some(Self {
            step,
            iter: self.iter,
            start,
            end,
        })
    }

    fn progress(&self) -> f32 {
        let now = Instant::now();
        if now >= self.end {
            1.0
        } else {
            let elapsed = now - self.start;
            let duration = self.end - self.start;
            elapsed.as_secs_f32() / duration.as_secs_f32()
        }
    }
}

struct MyApp {
    beap: Beap<u32>,
    last_beap_arr: Vec<u32>,
    animation_duration: f32,
    max_offset: f32,
    current_sequence: Option<SwapAnimationSequence>,
}

impl MyApp {
    fn regen_beap_arr(&mut self) {
        self.last_beap_arr = self.beap.iter().copied().collect();
    }
}

impl Default for MyApp {
    fn default() -> Self {
        let beap = Beap::new();

        Self {
            last_beap_arr: beap.iter().copied().collect(),
            beap,
            animation_duration: 0.5,
            max_offset: 0.0,
            current_sequence: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let everything_disabled = self.current_sequence.is_some();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_enabled_ui(!everything_disabled, |ui| {
                if ui.button("Add random").clicked() {
                    let next_number = rand::random::<u32>() % 100;

                    let iter = self
                        .beap
                        .insert_steps::<StepTracker<_>>(next_number)
                        .steps
                        .into_iter();

                    self.last_beap_arr.push(next_number);

                    self.current_sequence = SwapAnimationSequence::new(
                        Box::new(iter),
                        Duration::from_secs_f32(self.animation_duration),
                    );
                }

                if ui.button("Remove max").clicked() {
                    let iter = self
                        .beap
                        .pop_smallest_steps::<StepTracker<_>>()
                        .steps
                        .into_iter();

                    self.current_sequence = SwapAnimationSequence::new(
                        Box::new(iter),
                        Duration::from_secs_f32(self.animation_duration),
                    );
                }

                if ui.button("Remove random").clicked() {
                    let index = rand::random::<usize>() % self.beap.len();
                    let coord = BeapCoordinate::from_index(index);
                    let iter = self
                        .beap
                        .remove_steps::<StepTracker<_>>(coord)
                        .steps
                        .into_iter();

                    self.current_sequence = SwapAnimationSequence::new(
                        Box::new(iter),
                        Duration::from_secs_f32(self.animation_duration),
                    );
                }

                if ui.button("Increment random").clicked() {
                    // Index in upper half of array
                    let add = 50;
                    let index = rand::random::<usize>() % (self.beap.len() / 2);
                    self.last_beap_arr[index] += add;

                    let coord = BeapCoordinate::from_index(index);
                    let value = *self.beap.get_coord(coord).unwrap();
                    let iter = self
                        .beap
                        .set_value_steps::<StepTracker<_>>(coord, value)
                        .steps
                        .into_iter();

                    self.current_sequence = SwapAnimationSequence::new(
                        Box::new(iter),
                        Duration::from_secs_f32(self.animation_duration),
                    );
                }

                if ui.button("Decrement numbers above 50 by 50").clicked() {
                    // Index in upper half of array
                    let mut step_groups = vec![];

                    loop {
                        let above_50 = self
                            .beap
                            .find_smallest_item_greater_than_steps::<StepTracker<_>>(&50);
                        if let Some(coord) = above_50.result {
                            let value = *self.beap.get_coord(coord).unwrap();
                            let iter = self
                                .beap
                                .set_value_steps::<StepTracker<_>>(coord, value - 50)
                                .steps
                                .into_iter();

                            step_groups.push(iter);
                        } else {
                            break;
                        }
                    }

                    self.current_sequence = SwapAnimationSequence::new(
                        Box::new(step_groups.into_iter().flatten()),
                        Duration::from_secs_f32(self.animation_duration),
                    );
                }

                ui.label("Note: I got tired and didn't get around to the search animations");

                ui.add(
                    Slider::new(&mut self.animation_duration, 0.0..=0.5).text("Animation duration"),
                );
            });

            if let Some(seq) = self.current_sequence.as_ref() {
                ctx.request_repaint();
                if seq.progress() >= 1.0 {
                    let first_index = seq.step.first.array_index();
                    let second_index = seq.step.second.array_index();
                    if seq.step.overwrite {
                        self.last_beap_arr[second_index] = self.last_beap_arr[first_index];
                        self.last_beap_arr.remove(first_index);
                    } else {
                        self.last_beap_arr.swap(first_index, second_index);
                    }

                    // Step to next animation
                    self.current_sequence = self
                        .current_sequence
                        .take()
                        .unwrap()
                        .next(Duration::from_secs_f32(self.animation_duration));

                    if self.current_sequence.is_none() {
                        self.regen_beap_arr();
                    }
                }
            }

            let h_spacing = 80.0f32;
            let v_spacing = h_spacing * (1.0f32 - 0.5f32 * 0.5f32).sqrt();

            let padding = 50.0;

            let beap_width = self.beap.depth().max(6) as f32 * h_spacing;

            let beap_center = (beap_width / 2.0).max(self.max_offset);
            self.max_offset = beap_center;

            let (rect, _) = ui.allocate_exact_size(ui.available_size(), Sense::click());

            let get_pos_for_coordinate = |coord: BeapCoordinate| {
                let x = padding + rect.left() + beap_center + (coord.pos() as f32 * h_spacing)
                    - coord.row() as f32 * h_spacing / 2.0;
                let y = padding + rect.top() + coord.row() as f32 * v_spacing;
                Pos2::new(x, y)
            };

            let is_valid_coord =
                |coord: BeapCoordinate| coord.array_index() < self.last_beap_arr.len();

            for i in 0..self.last_beap_arr.len() {
                let coord = BeapCoordinate::from_index(i);

                let draw_child_line = |child: BeapCoordinate| {
                    if is_valid_coord(child) {
                        let child_pos = get_pos_for_coordinate(child);
                        let parent_pos = get_pos_for_coordinate(coord);
                        ui.painter().line_segment(
                            [parent_pos, child_pos],
                            egui::Stroke::new(1.0, egui::Color32::LIGHT_GRAY),
                        );
                    }
                };
                draw_child_line(coord.left_child());
                draw_child_line(coord.right_child());
            }

            for i in 0..self.last_beap_arr.len() {
                let coord = beap::BeapCoordinate::from_index(i);
                let mut pos = get_pos_for_coordinate(coord);

                if let Some(seq) = self.current_sequence.as_ref() {
                    let other_coord: Option<BeapCoordinate>;
                    if coord == seq.step.first {
                        other_coord = Some(seq.step.second);
                    } else if coord == seq.step.second {
                        other_coord = Some(seq.step.first);
                    } else {
                        other_coord = None;
                    }

                    if let Some(other_coord) = other_coord {
                        let other_pos = get_pos_for_coordinate(other_coord);
                        let progress = seq.progress();

                        fn lerp(a: f32, b: f32, t: f32) -> f32 {
                            a + (b - a) * t
                        }

                        pos = Pos2::new(
                            lerp(pos.x, other_pos.x, progress),
                            lerp(pos.y, other_pos.y, progress),
                        );
                    }
                }

                let rect = egui::Rect::from_center_size(pos, egui::Vec2::new(30.0, 30.0));
                let text_col = ui.style().visuals.text_color();
                ui.painter().rect(
                    rect,
                    3.0,
                    ui.style().visuals.faint_bg_color,
                    (2.0, text_col),
                );
                ui.painter().text(
                    pos,
                    Align2::CENTER_CENTER,
                    self.last_beap_arr[i],
                    FontId::default(),
                    text_col,
                );
            }
        });
    }
}
