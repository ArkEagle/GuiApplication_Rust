#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use eframe::egui::{Pos2, Response, Ui};
use eframe::egui::plot::PlotPoint;
use eframe::epaint::RectShape;

fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    name: String,
    age: u32,
    click : bool,
    square_pos : Pos2,
    frame : egui::epaint::RectShape,
    square_name_vec : Vec<[char;128]>,
    square_rec_vec : Vec<egui::epaint::RectShape>,
    square_respone_vec : Vec<Response>,


}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            click : false,
            square_pos : egui::Pos2{x:40.0, y:40.0},

            frame : egui::epaint::RectShape{
                rect: egui::Rect{min: egui::Pos2 { x:40.0, y :40.0 },max: egui::Pos2{ x:60.0, y :60.0 }},
                rounding: egui::Rounding{nw:0.2,ne:0.2,sw:0.2,se:0.2},
                fill: egui::Color32::from_rgb(255, 128, 128),
                stroke: egui::Stroke::NONE
            },
            square_name_vec : vec![],
            square_rec_vec : vec![],
            square_respone_vec : vec![],

        }
    }
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.frame.rect.min = self.square_pos;
        self.frame.rect.max = egui::Pos2 {x:self.square_pos.x+20.0,y:self.square_pos.y+20.0};
        egui::TopBottomPanel::top("Testpanel").show(ctx, |ui|{
            ui.horizontal(|ui|{
                if ui.button("Generate Square").clicked() {
                    self.square_rec_vec.push(self.frame);
                    self.square_respone_vec.push(ui.allocate_rect(egui::Rect{min: self.frame.rect.min,max: self.frame.rect.max},egui::Sense::drag()))
                }

            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.click {
                ui.heading("My egui Application");
            }
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
            let mut delet_vec : Vec<RectShape> = Vec::new();
            for i_frame in self.square_rec_vec.iter_mut(){
                ui.painter().add(*i_frame);
                let i_r = ui.allocate_rect(i_frame.rect,egui::Sense::drag());
                if i_r.clicked() {
                    self.click = !self.click;
                }
                if i_r.dragged(){
                    let delta = i_r.drag_delta();
                    i_frame.rect.min = Pos2{x: i_frame.rect.min.x+delta[0],y: i_frame.rect.min.y+delta[1]};
                    i_frame.rect.max = Pos2{x: i_frame.rect.max.x+delta[0],y: i_frame.rect.max.y+delta[1]};
                }/*
                if i_r.secondary_clicked(){
                    delet_vec.push(*i_frame)
                }*/
                i_r.context_menu(|ui|{
                    if ui.button("Delete").clicked(){
                        delet_vec.push(*i_frame);
                        ui.close_menu();
                    }
                });
            }

            for i_frame in delet_vec{
                self.square_rec_vec.retain(|&x| x!=i_frame);
            }
            ui.label(format!("Laenge des Vektors '{}'", self.square_rec_vec.len()));

            /* Working Code
            ui.painter().add(self.frame);
            let r = ui.allocate_rect(egui::Rect{min: self.frame.rect.min,max: self.frame.rect.max},egui::Sense::drag());
            if r.clicked() {
                self.click = !self.click;
            }
            if r.dragged(){
                let delta = r.drag_delta();
                self.square_pos.x = self.square_pos.x+delta[0];
                self.square_pos.y = self.square_pos.y+delta[1];
                ui.label(format!("X-Drag '{}', Y-Drag {}", delta[0], delta[1]));
            }
            */
            /*ui.painter().rect(
                egui::Rect{min: egui::Pos2 { x:10.0, y :10.0 },max: egui::Pos2{ x:20.0, y :20.0 }},
                egui::Rounding{nw:0.2,ne:0.2,sw:0.2,se:0.2},
                egui::Color32::from_rgb(255, 128, 128),
                egui::Stroke::NONE
            )*/
        });

    }
}

impl MyApp {
    fn rightclickMenu(&mut self, ui: &mut egui::Ui,frame: egui::epaint::RectShape) {
        if ui.button("Test").clicked(){
            ui.close_menu();
        }
    }
}