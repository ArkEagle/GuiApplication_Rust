#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod backend;

use std::fmt::format;
use backend::clickedIO;
use eframe::egui;
use eframe::egui::{Pos2, Response, SidePanel, Ui};
use eframe::egui::plot::PlotPoint;
use eframe::epaint::RectShape;


fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1920.0, 1080.0)),
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
    state_vec : Vec<backend::State>,
    sidebar_enabled : bool,
    n_state : u8,
    init_state : backend::State,
    clicked_new_state : bool,
    NewState : New_state_input,
    clickedIO : Option<backend::clickedIO>,
    IOPair_vec : Vec<[backend::clickedIO;2]>,
    IOPair : [backend::clickedIO;2],
    init_connect : bool,
    complete_connect : bool,
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
            state_vec : vec![],
            sidebar_enabled : false,
            n_state : 0,
            init_state : backend::State::new(5, 2, "".parse().unwrap(), "".parse().unwrap(), 0),/*backend::State {
                I: backend::IO { Type: backend::IoType::Input, IOVec: Vec::with_capacity(0) },
                O: backend::IO { Type: backend::IoType::Input, IOVec: Vec::with_capacity(0) },
                Name : ("").parse().unwrap(),
                ID : 0,
                content : "".parse().unwrap(),
                frame : egui::epaint::RectShape{
                        rect: egui::Rect{min: egui::Pos2 { x:40.0, y :40.0 },max: egui::Pos2{ x:100.0, y :80.0 }},
                        rounding: egui::Rounding{nw:0.2,ne:0.2,sw:0.2,se:0.2},
                        fill: egui::Color32::from_rgb(255, 128, 128),
                        stroke: egui::Stroke::NONE
                        },

            }*/
            clicked_new_state : false,
            NewState : New_state_input{
                n_Input : 0,
                n_Output : 0,
                title : "".parse().unwrap(),
                content : "".parse().unwrap(),

            },
            clickedIO : None,
            IOPair : [backend::clickedIO{IOType : backend::IoType::Input,IO_number : 0,State : 0},backend::clickedIO{IOType : backend::IoType::Input,IO_number : 0,State : 0}],
            IOPair_vec : Vec::new(),
            init_connect : false,
            complete_connect : false,
            
        }
    }
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.frame.rect.min = self.square_pos;
        self.frame.rect.max = egui::Pos2 {x:self.square_pos.x+20.0,y:self.square_pos.y+20.0};
        if self.clicked_new_state {
            self.newState(ctx);

        }
        //====================Top Panel====================
        egui::TopBottomPanel::top("Testpanel").show(ctx, |ui|{
            ui.horizontal(|ui|{
                if ui.button("Generate Square").clicked() {
                    self.n_state += 1;
                    let mut state = self.init_state.clone();
                    state.ID = self.n_state;
                    //self.square_rec_vec.push(self.frame);
                    self.state_vec.push(state);

                }
                if ui.button("Testbutton Window").clicked(){
                    self.n_state += 1;
                    let mut state = self.init_state.clone();
                    self.clicked_new_state = true;
                }

            });
        });
        //====================Central Panel====================

        egui::CentralPanel::default().show(ctx, |ui| {
            /*
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
            */

            let mut delet_vec : Vec<backend::State> = Vec::new();
            /*
            for i_frame in self.square_rec_vec.iter_mut(){
                //===========State Painting===========
                ui.painter().add(*i_frame);
                let i_r = ui.allocate_rect(i_frame.rect,egui::Sense::drag());
                //===========State Interaction===========
                if i_r.clicked() {
                    self.sidebar_enabled = true;
                }
                if i_r.dragged(){
                    let delta = i_r.drag_delta();
                    i_frame.rect.min = Pos2{x: i_frame.rect.min.x+delta[0],y: i_frame.rect.min.y+delta[1]};
                    i_frame.rect.max = Pos2{x: i_frame.rect.max.x+delta[0],y: i_frame.rect.max.y+delta[1]};
                }
                if i_r.clicked_elsewhere(){
                    self.sidebar_enabled = false;
                }
                /*
                if i_r.secondary_clicked(){
                    delet_vec.push(*i_frame)
                }*/
                i_r.context_menu(|ui|{
                    if ui.button("Delete").clicked(){
                        delet_vec.push(*i_frame);
                        ui.close_menu();
                    }
                });
            };
            */
            let mut clicked_inframe : bool = false;
            for i_state in self.state_vec.iter_mut(){
                //===========State Painting===========

                ui.painter().add(i_state.frame);
                i_state.DrawTitle(ui,ctx);

                let i_r = ui.allocate_rect(i_state.frame.rect,egui::Sense::drag());
                //===========State Interaction===========
                if i_r.clicked() {
                    self.sidebar_enabled = true;
                    clicked_inframe = true;
                }
                if i_r.dragged(){
                    let delta = i_r.drag_delta();
                    i_state.frame.rect.min = Pos2{x: i_state.frame.rect.min.x+delta[0],y: i_state.frame.rect.min.y+delta[1]};
                    i_state.frame.rect.max = Pos2{x: i_state.frame.rect.max.x+delta[0],y: i_state.frame.rect.max.y+delta[1]};
                    i_state.IO_anker_template.center = Pos2{x: i_state.IO_anker_template.center.x+delta[0], y: i_state.IO_anker_template.center.y+delta[1]}
                }
                if i_r.clicked_elsewhere() & !clicked_inframe{
                    self.sidebar_enabled = false;
                }
                /*
                if i_r.secondary_clicked(){
                    delet_vec.push(*i_frame)
                }*/
                i_r.context_menu(|ui|{
                    if ui.button("Delete").clicked(){
                        delet_vec.push(i_state.clone());
                        ui.close_menu();
                    };
                    ui.text_edit_singleline(&mut self.name);
                });
                self.clickedIO = i_state.Draw_IO(ui);
                match &self.clickedIO {
                    None => {;},
                    Some(click_IO) => {
                        self.init_connect = true;
                        match click_IO.IOType {
                            backend::IoType::Input => self.IOPair[0] = click_IO.clone(),
                            backend::IoType::Output => self.IOPair[1] = click_IO.clone()
                        }
                    }
                    
                }

            };
            for i_state in delet_vec {
                self.state_vec.retain(|x| x != &i_state);
            }

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

        //====================Right Sidepanel====================
        if self.sidebar_enabled{
            egui::SidePanel::right("Propotries").show(ctx,|ui|{
                if ui.button("Test").clicked(){
                    ;
                }
            });
        }

    }
}

impl MyApp {
    fn newState(&mut self, ctx : &egui::Context) {
        egui::Window::new("Parameter für den neuen Zustand").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.label(format!("Zustands-ID: {}",self.n_state));
                ui.horizontal(|ui|{
                    ui.label("Zustandsname: ");
                    ui.text_edit_singleline(&mut self.NewState.title);
                });
                ui.add(egui::Slider::new(&mut self.NewState.n_Input, 0..=10).text("Anzahl der Inputs"));
                ui.add(egui::Slider::new(&mut self.NewState.n_Output, 0..=10).text("Anzahl der Outputs"));
                ui.label("Zustandsinhalt: ");
                ui.text_edit_multiline(&mut self.NewState.content).on_hover_text("Hier durchzuführende Funktionen angeben");
                ui.horizontal(|ui|{
                    if ui.button("Close window").clicked(){
                        self.clicked_new_state =false;
                        self.n_state -=1;
                    };
                    if ui.button("Create State").clicked(){
                        let state : backend::State = backend::State::new(self.NewState.n_Input,self.NewState.n_Output,self.NewState.title.clone(),self.NewState.content.clone(),self.n_state);
                        self.state_vec.push(state);
                        self.NewState = New_state_input{
                            n_Input : 0,
                            n_Output : 0,
                            title : "".parse().unwrap(),
                            content : "".parse().unwrap(),
                        };
                        self.clicked_new_state =false;
                    };

                });
            });


        });
    }
}

struct New_state_input{
    n_Input : usize,
    n_Output : usize,
    title : String,
    content : String,
}