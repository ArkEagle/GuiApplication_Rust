#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
//TODO: eventuell Sprungmarken hinzufügen
//TODO: Systematische Simulation (Recherche: zB Sackgassen)
mod backend;

use backend::{clickedIO, State};
use eframe::egui;
use eframe::egui::{Event, Pos2, Response, SidePanel, TextBuffer, Ui};
use eframe::egui::plot::PlotPoint;
use eframe::epaint::RectShape;
use std::{fs, path, io, backtrace};
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::ops::Add;
use serde::{Serialize,Deserialize};
use serde_json;
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
    LineStart : egui::Pos2,
    io_draw_template : egui::epaint::CircleShape,
    start_state_exists : bool,
    selected_state : Option<backend::State>,
    selected_state_Output_Con : Vec<String>,
    set_ouput_con : bool,
    ChangedState : New_state_input,
    filename : String,
    in_saving : bool,
    in_loading : bool,
    scale : f32,
    min_scale : f32,
    max_scale : f32,
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
            init_state : backend::State::new(5, 2, "".parse().unwrap(), "".parse().unwrap(), 0,false, 1.0),/*backend::State {
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
                n_Input : 1,
                n_Output : 1,
                title : "".parse().unwrap(),
                content : "".parse().unwrap(),
                is_start_state : false,
                ConVec : vec![String::from("");1]

            },
            clickedIO : None,
            IOPair : [backend::clickedIO{IOType : backend::IoType::Input,IO_number : 0,State : 0},backend::clickedIO{IOType : backend::IoType::Input,IO_number : 0,State : 0}],
            IOPair_vec : Vec::new(),
            init_connect : false,
            complete_connect : false,
            LineStart : egui::Pos2{x:40.0,y:40.0},
            io_draw_template : egui::epaint::CircleShape{
                center: egui::Pos2{x:44.0, y :46.0},
                radius:3.0,
                fill: egui::Color32::from_rgb(96, 96, 96),
                stroke:egui::Stroke{width: 1.0,color: egui::Color32::from_rgb(220, 220, 220)
                }},
            start_state_exists : false,
            selected_state : None,
            selected_state_Output_Con : Vec::new(),
            set_ouput_con : false,
            ChangedState : New_state_input{
                n_Input : 1,
                n_Output : 1,
                title : "".parse().unwrap(),
                content : "".parse().unwrap(),
                is_start_state : false,
                ConVec : vec![String::from("");1]},
            filename : String::from(""),
            in_saving : false,
            in_loading : false,
            scale : 1.0,
            min_scale : 0.3,
            max_scale : 2.0,
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
        if self.set_ouput_con{
            
            self.setOutputCondition(ctx);
        }
        if self.in_saving{
            self.saveStateMachine(ctx);
        }
        if self.in_loading{
            self.loadStateMachine(ctx);
        }

        //====================Central Panel====================

        egui::CentralPanel::default().show(ctx, |ui| {
            let mut zoom = 1.0;
            if self.scale >= self.min_scale{
                zoom = ui.input(|i| i.zoom_delta());
                self.scale = zoom*self.scale;
            }
            else if self.scale < self.min_scale {
                zoom = 1.0;
                self.scale = self.min_scale;
            }

            if ui.input(|i|{i.modifiers.ctrl}){
                ui.label(zoom.to_string());

                ui.label(self.scale.to_string());
            }

            let mut delet_vec : Vec<backend::State> = Vec::new();
            let mut clicked_inframe : bool = false;
            let mut clicked_io_b : bool = false;
            let scrollDelta = ui.input(|i| i.scroll_delta);
            for i_state in self.state_vec.iter_mut(){
                //===========State Painting===========
                match &self.selected_state {
                    None => {},
                    Some(state) => {
                        if state.ID == i_state.ID {
                            let mut highlight = i_state.frame.clone();
                            highlight.fill = egui::Color32::from_rgba_premultiplied(0, 0, 0, 0);
                            highlight.stroke = egui::epaint::Stroke{ width: 2.0, color: egui::Color32::from_rgb(220, 220, 220) };
                            ui.painter().add(highlight);
                        }
                    }
                };

                i_state.Draw_Box(ui,self.scale,scrollDelta);
                i_state.DrawTitle(ui, self.scale,);
                i_state.DrawContent(ui, self.scale,);
                let i_r = ui.allocate_rect(i_state.frame.rect,egui::Sense::drag());
                self.clickedIO = i_state.Draw_IO(ui, &self.scale,);
                //clicks auswerten
                match &self.clickedIO {
                    None => {;},
                    Some(click_IO) => {
                        clicked_io_b = true;
                        self.init_connect = true;
                        match click_IO.IOType {
                            backend::IoType::Input => self.IOPair[0] = click_IO.clone(),
                            backend::IoType::Output => self.IOPair[1] = click_IO.clone()
                        }
                    }
                }
                if clicked_io_b{
                    ui.label("Click registiert!");
                }
                //===========State Interaction===========
                if i_r.clicked() {
                    self.sidebar_enabled = true;
                    clicked_inframe = true;
                    self.selected_state = Some(i_state.clone());
                }
                if i_r.dragged(){
                    let delta = i_r.drag_delta();
                    i_state.frame.rect.min = Pos2{x: i_state.frame.rect.min.x+delta[0],y: i_state.frame.rect.min.y+delta[1]};
                    i_state.frame.rect.max = Pos2{x: i_state.frame.rect.max.x+delta[0],y: i_state.frame.rect.max.y+delta[1]};
                    i_state.IO_anker_template.center = Pos2{x: i_state.IO_anker_template.center.x+delta[0], y: i_state.IO_anker_template.center.y+delta[1]}
                }
                if i_r.clicked_elsewhere() & !clicked_inframe{
                    //self.selected_state = None;

                }
                i_r.context_menu(|ui|{
                    if ui.button("Delete").clicked(){
                        delet_vec.push(i_state.clone());
                        ui.close_menu();
                    };
                    ui.text_edit_singleline(&mut self.name);
                });
                if ui.input(|i|i.key_pressed(egui::Key::Escape)){
                    self.init_connect = false;
                    //Reset Connection-Prozess
                        self.IOPair[0] = backend::clickedIO {
                            IOType: backend::IoType::Input,
                            IO_number: 0,
                            State: 0,
                        };
                        self.IOPair[1] = backend::clickedIO {
                            IOType: backend::IoType::Input,
                            IO_number: 0,
                            State: 0,
                        };
                }
                //=========IO-Drawing und Handling von Verbindungen=========
                //clicks weiterverarbeiten
                if self.init_connect{
                    if (self.IOPair[0].State != 0 as u8 && self.IOPair[1].State == 0 as u8){
                        if i_state.ID == self.IOPair[0].State{
                            self.LineStart =egui::Pos2{x: i_state.frame.rect.min.x+4.0,y: i_state.frame.rect.min.y + (self.IOPair[0].IO_number as f32*10.0+4.0)*self.scale} ;
                        }
                        //let IO_Pos = self.state_vec[State_pos];
                        }
                    else if (self.IOPair[0].State == 0 as u8 && self.IOPair[1].State != 0 as u8){
                        if i_state.ID == self.IOPair[1].State{
                            self.LineStart =egui::Pos2{x: i_state.frame.rect.max.x-4.0,y: i_state.frame.rect.min.y + (self.IOPair[1].IO_number as f32*10.0+4.0)*self.scale} ;
                        }
                    }
                    else if (self.IOPair[0].State != 0 as u8 && self.IOPair[1].State != 0 as u8) {
                        if self.IOPair_vec.contains(&self.IOPair){

                            self.IOPair_vec.retain(|x| x != &self.IOPair);
                        }
                        else {
                            self.IOPair_vec.push(self.IOPair.clone());
                        }
                        
                        self.IOPair[0] = backend::clickedIO {
                            IOType: backend::IoType::Input,
                            IO_number: 0,
                            State: 0,
                        };
                        self.IOPair[1] = backend::clickedIO {
                            IOType: backend::IoType::Input,
                            IO_number: 0,
                            State: 0,
                        };
                        self.init_connect = false;
                    }
                    //zeichnen der aktuell zu ziehenden Verbindung
                match ctx.pointer_hover_pos(){
                    None => {},
                    Some(Pos) => {
                        let Line = egui::epaint::Shape::LineSegment{points:[Pos,self.LineStart],stroke: egui::Stroke{width:2.0*self.scale,color : egui::Color32::from_rgb(255, 128, 128)}};
                        ui.painter().add(Line);
                    }
                }
            }

            };//End of Loop
            //zeichnen von bestehenden Verbindungen
            for connection in &self.IOPair_vec{
                let index_input = self.state_vec.iter().position(|input_state| input_state.ID == connection[0].State  ).unwrap();
                let index_output = self.state_vec.iter().position(|output_state| output_state.ID == connection[1].State  ).unwrap();
                let Pos_input = Pos2{x: self.state_vec[index_input].frame.rect.min.x+4.0*self.scale,y:self.state_vec[index_input].frame.rect.min.y+(connection[0].IO_number as f32*10.0+4.0)*self.scale};
                let Pos_output = Pos2{x: self.state_vec[index_output].frame.rect.max.x-4.0*self.scale,y:self.state_vec[index_output].frame.rect.min.y+(connection[1].IO_number as f32*10.0+4.0)*self.scale};
                let line = egui::epaint::Shape::LineSegment { points: [Pos_input,Pos_output], stroke: egui::Stroke{width : 2.0*self.scale, color : egui::Color32::from_rgb(220, 220, 220)} };
                ui.painter().add(line);
            }   
            //löschen von zu löschenden Zuständen
            for i_state in delet_vec {
                if i_state.isStart{
                    self.start_state_exists = false;
                }
                self.IOPair_vec.retain(|x| x[0].State !=i_state.ID && x[1].State !=i_state.ID);
                self.state_vec.retain(|x| x != &i_state);
            }
            //

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

        //====================Top Panel====================

        egui::TopBottomPanel::top("CreatePanel").exact_height(20.0).show(ctx, |ui|{
            ui.horizontal(|ui|{
                if self.init_connect{
                    ui.label("Verlassen des Verbindungsmodus ESC");
                }
                if ui.button("ZA speichern").clicked(){
                    self.in_saving = true;
                }
                if ui.button("ZA laden").clicked(){
                    self.in_loading = true;
                }
                if ui.button("Generate Square").clicked() {
                    self.n_state += 1;
                    let mut state = self.init_state.clone();
                    self.clicked_new_state = true;
                }
                if ui.button("Testbutton Window").clicked(){
                    ui.label("Aktuell keine Funktion");
                }

            });
        });

        //====================Right Sidepanel====================
        match &mut self.selected_state{
            None => {},
            Some(state) =>{
                self.SidepanelStateConfig(ctx);
            }
        }
        //====================Bottompanel====================
        egui::TopBottomPanel::bottom("Meldungspanel").show(ctx, |ui|{
            ui.vertical(|ui|{
                ui.heading("Statusmeldungen");
                ui.separator();
                if !self.start_state_exists{
                    ui.label(egui::RichText::from("Startstate fehlt").size(12.0).color(egui::epaint::Color32::from_rgb(170, 0, 0)));
                }
            }
            )
        });
    }
}

impl MyApp {
    fn newState(&mut self, ctx : &egui::Context) {
        egui::Window::new("Zustandsautomat speichern").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.label(format!("Zustands-ID: {}",self.n_state));
                ui.horizontal(|ui|{
                    ui.label(format!("Zustandsname: "));
                    ui.text_edit_singleline(&mut self.NewState.title);
                });
                ui.add_enabled(!self.start_state_exists, egui::Checkbox::new(&mut self.NewState.is_start_state, "Soll als Start festgelegt werden"));
                if self.start_state_exists{
                    ui.label("Startzusand wurde bereits festgelegt.").on_hover_text("Sie haben bereits den Startzustand vergeben. \n Sie erkennen ihn an dem Highlight. \n Löschen sie den aktuellen Startzustand, um einen neuen Startzustand vergeben zu können.");
                }
                if self.NewState.is_start_state {
                    ui.add(egui::Slider::new(&mut self.NewState.n_Input, 0..=10).text("Anzahl der Inputs. Minimum 0!"));

                }
                else{
                    ui.add(egui::Slider::new(&mut self.NewState.n_Input, 1..=10).text("Anzahl der Inputs"));
                }
                
                ui.add(egui::Slider::new(&mut self.NewState.n_Output, 1..=10).text("Anzahl der Outputs"));
                
                ui.label("Zustandsinhalt: ");
                ui.text_edit_multiline(&mut self.NewState.content).on_hover_text("Hier durchzuführende Funktionen angeben");
                ui.horizontal(|ui|{
                    if ui.button("Close window").clicked(){
                        self.clicked_new_state =false;
                        self.n_state -=1;
                    };
                    if ui.button("Create State").clicked(){
                        if self.NewState.is_start_state{
                            self.start_state_exists = true;
                        }
                        let state : backend::State = backend::State::new(self.NewState.n_Input,self.NewState.n_Output,self.NewState.title.clone(),self.NewState.content.clone(),self.n_state,self.NewState.is_start_state, self.scale);
                        self.selected_state = Some(state.clone());
                        self.state_vec.push(state);
                        self.NewState = New_state_input{
                            n_Input : 1,
                            n_Output : 1,
                            title : "".parse().unwrap(),
                            content : "".parse().unwrap(),
                            is_start_state : false,
                            ConVec : vec![String::from("");1],
                        };
                        self.clicked_new_state =false;
                        self.set_ouput_con = true;
                    };

                });
            });


        });}
    fn setOutputCondition(&mut self, ctx :&egui::Context){
        match &mut self.selected_state {
            None => {},
            Some(state) => {
                egui::Window::new("Setzen der Transitionsbedingungen").show(ctx,|ui|{
                    ui.vertical(|ui|{
                        for (n,OutCond) in state.O_con_vec.iter_mut().enumerate(){
                            ui.horizontal(|ui|{
                                ui.label(format!("Transitionsbedingung für Transition {}",n+1));
                                ui.text_edit_singleline(OutCond);
                            });
                        }
                    });
            ui.horizontal(|ui| {
                if ui.button("Exit").clicked(){
                    self.set_ouput_con = false;
                }
                else if ui.button("Transitionen bestätigen").clicked(){
                    for filtered_state in self.state_vec.iter_mut().filter(|state_in_vec| state_in_vec.ID==state.ID){
                        filtered_state.O_con_vec = state.O_con_vec.clone();
                    }
                    self.set_ouput_con = false;

                }
            });
            
        });
    }}
        
    } 
    fn SidepanelStateConfig(&mut self, ctx: &egui::Context){
        //TODO: Mit Hilfsvariable self.changedState des typs newState arbeiten
        egui::SidePanel::right("Properties").show(ctx,|ui|{
            //self.ChangedState.n_Input = self.selected_state.as_ref().unwrap().I.IOVec.len();
            //self.ChangedState.n_Output = self.selected_state.as_ref().unwrap().O.IOVec.len();
                ui.vertical(|ui| {
                    ui.label(format!("Zustands-ID: {}",self.selected_state.as_ref().unwrap().ID));
                    ui.vertical(|ui|{
                        ui.label(format!("Zustandsname: {}",self.selected_state.as_ref().unwrap().Name));
                        ui.text_edit_singleline(&mut self.ChangedState.title);
                    });
                    /* TODO: Lösung suchen
                    ui.add_enabled(!self.start_state_exists, egui::Checkbox::new( &mut self.ChangedState.is_start_state, "Soll als Start festgelegt werden"));
                    */
                    if self.start_state_exists{
                        ui.label("Startzusand wurde bereits festgelegt.").on_hover_text("Sie haben bereits den Startzustand vergeben. \n Sie erkennen ihn an dem Highlight. \n Löschen sie den aktuellen Startzustand, um einen neuen Startzustand vergeben zu können.");
                    }

                    if self.ChangedState.is_start_state  || self.selected_state.as_ref().unwrap().isStart{
                        ui.add(egui::Slider::new(&mut self.ChangedState.n_Input, 0..=10).text("Anzahl der Inputs. Minimum 0!"));
    
                    }
                    else{
                        ui.add(egui::Slider::new(&mut self.ChangedState.n_Input, 1..=10).text("Anzahl der Inputs"));
                    }
                    
                    ui.add(egui::Slider::new(&mut self.ChangedState.n_Output, 1..=10).text("Anzahl der Outputs"));
                    
                    ui.label("Zustandsinhalt: ");
                    ui.text_edit_multiline(&mut self.ChangedState.content).on_hover_text("Zustandsablauf");
                    ui.horizontal(|ui| {
                       if ui.button("Schließen").clicked(){
                           self.ChangedState = New_state_input{
                               n_Input : 1,
                               n_Output : 1,
                               title : "".parse().unwrap(),
                               content : "".parse().unwrap(),
                               is_start_state : false,
                               ConVec : vec![String::from("");1]};

                           self.selected_state = None;
                       }

                       if ui.button("Änderungen speichern").clicked(){
                           for filtered_state in self.state_vec.iter_mut().filter(|state_in_vec| state_in_vec.ID==self.selected_state.as_ref().unwrap().ID){
                               if self.ChangedState.title ==String::from(""){
                                   self.ChangedState.title = self.selected_state.clone().unwrap().Name;
                               }
                               filtered_state.refactorState(self.ChangedState.n_Input,self.ChangedState.n_Output,self.ChangedState.title.clone(),self.ChangedState.content.clone(),self.ChangedState.is_start_state);
                           }
                           self.ChangedState = New_state_input{
                               n_Input : 1,
                               n_Output : 1,
                               title : "".parse().unwrap(),
                               content : "".parse().unwrap(),
                               is_start_state : false,
                               ConVec : vec![String::from("");1]};
                           if self.ChangedState.is_start_state{
                               self.start_state_exists = true;

                           }
                           if self.selected_state.as_ref().unwrap().isStart && !self.ChangedState.is_start_state{
                               self.start_state_exists = false;
                           }
                           if self.selected_state.as_ref().unwrap().O.IOVec.len()!=self.ChangedState.n_Output{
                               self.set_ouput_con = true;
                           }
                           else{
                               self.selected_state = None;
                           }
                       }

                    });
            });
    });
}
    fn saveStateMachine(&mut self, ctx: &egui::Context, ){
        let PathString = "./SystemStorage";
        let mut init_write = false;
        egui::Window::new("Parameter für den neuen Zustand").collapsible(false).open(&mut self.in_saving).show(ctx, |ui| {
            egui::ScrollArea::vertical().max_height(150.0).show(ui,|ui|{
                ui.vertical(|ui| {
                    for entry in fs::read_dir(path::Path::new(PathString)).unwrap() {
                        let file_entry = String::from(entry.unwrap().file_name().to_str().unwrap());
                        if ui.button(&file_entry).clicked() {
                            //let entry = entry.unwrap().file_name();
                            self.filename = file_entry;
                        }
                    }
                });
            });

            ui.separator();
            ui.horizontal(|ui|{
                ui.text_edit_singleline( &mut self.filename);
                if ui.button("Speichern").clicked() /*&& self.filename == String::from("")*/{
                    init_write = true;
                }
            });
        });
        if init_write{
            self.writeFileStateMachine();

            init_write = false;
        }
    }
    fn writeFileStateMachine_selfserial(&mut self){
        let PathString = "./SystemStorage/";
        let mut file = fs::File::create((String::from(PathString)+ self.filename.as_str()).as_str()).unwrap();
        let mut content = String::from("IOPair_vec: \n");
        //=====State-Verbindungsvektor schreiben=====
        for i in &self.IOPair_vec{
            //erster State
            let mut PairString0 = String::from("\t\t[[")+ &*i[0].State.to_string()+ &*String::from(",") +&*i[0].IO_number.to_string()+ &*String::from(",");
            if i[0].IOType ==backend::IoType::Input{
                PairString0.push_str("Input");
            }
            else if i[0].IOType ==backend::IoType::Output {
                PairString0.push_str("Output");
            }
            PairString0.push_str("]");
            //zweiter State
            let mut PairString1 = String::from(",[")+ &*i[1].State.to_string()+ &*String::from(",") +&*i[1].IO_number.to_string()+ &*String::from(",");
            if i[1].IOType ==backend::IoType::Input{
                PairString1.push_str("Input");
            }
            else if i[1].IOType ==backend::IoType::Output {
                PairString1.push_str("Output");
            }
            PairString1.push_str("]]\n");
            content.push_str(&*PairString0);
            content.push_str(&*PairString1);
        }
        content.push_str("\n");
        content.push_str("state_vec: \n");
        //=====Statevektor schreiben=====
        for state in &self.state_vec{
            let mut StateString = String::from("ID: ")+ &*state.ID.to_string();
            //Input- und Outputanzahl schreiben
            StateString.push_str(";\n Inputnumber: ");
            let I_len = state.I.IOVec.len();
            StateString = StateString+&*I_len.to_string();
            StateString.push_str(";\n Outputnumber: ");
            let O_len = state.I.IOVec.len();
            StateString = StateString+&*O_len.to_string();
            
            //Outputbedingungen schreiben
            StateString.push_str(";\n  Ouputcondition: [");
            for cond in &state.O_con_vec{
                StateString.push_str(&*cond);
                StateString.push_str(&*String::from(","));
            }
            StateString.push_str("];\n");
            StateString.push_str("Name: ");
            StateString.push_str(&*state.Name);

        }
        file.write(content.clone().as_ref());
    }
    fn writeFileStateMachine(&mut self){
        let PathString = "./SystemStorage/";
        let mut file = fs::File::create((String::from(PathString)+ self.filename.as_str()).as_str()).unwrap();
        let mut content : String = serde_json::to_string(&self.state_vec).unwrap();
        content.push_str(&*String::from("\n"));
        content.push_str(&*serde_json::to_string(&self.IOPair_vec).unwrap());
        file.write(content.clone().as_ref());
    }

    fn loadStateMachine(&mut self, ctx: &egui::Context, ){
        let PathString = "./SystemStorage";
        let mut init_read = false;
        egui::Window::new("Zustandsautomat laden").collapsible(false).open(&mut self.in_loading).show(ctx, |ui| {
            egui::ScrollArea::vertical().max_height(150.0).show(ui,|ui|{
                ui.vertical(|ui| {
                    for entry in fs::read_dir(path::Path::new(PathString)).unwrap() {
                        let file_entry = String::from(entry.unwrap().file_name().to_str().unwrap());
                        if ui.button(&file_entry).clicked() {
                            //let entry = entry.unwrap().file_name();
                            self.filename = file_entry;
                        }
                    }
                });
            });

            ui.separator();
            ui.horizontal(|ui|{
                ui.label( &self.filename);
                if ui.button("Laden").clicked() /*&& self.filename == String::from("")*/{
                    init_read = true;
                }
            });
        });
        if init_read{
            self.readFileStateMachine();
            init_read = false;
        }
    }
    fn readFileStateMachine(&mut self){
        let PathString = "./SystemStorage/";
        let mut file = fs::File::open((String::from(PathString)+ self.filename.as_str()).as_str()).unwrap();
        let content = fs::read_to_string((String::from(PathString)+ self.filename.as_str())).expect("Error");
        if content != String::from("Error"){
            let mut lines =  content.lines();
            let state_vec : Vec<backend::State> = serde_json::from_str(&lines.next().unwrap()).unwrap();

            let IOPair_vec : Vec<[backend::clickedIO;2]> = serde_json::from_str(&lines.next().unwrap()).unwrap();

            self.state_vec = state_vec;
            self.IOPair_vec = IOPair_vec;
            self.n_state = 0;
            for state in self.state_vec.iter(){
                if self.n_state<state.ID{
                    self.n_state=state.ID
                }
                if state.isStart{
                    self.start_state_exists = true;

                }
            }

        }

    }
}


struct New_state_input{
    n_Input : usize,
    n_Output : usize,
    title : String,
    content : String,
    is_start_state : bool,
    ConVec : Vec<String>
}