use egui;
use eframe;
use epaint;


pub(crate) struct State {
    pub(crate) I : IO,
    pub(crate) O : IO,
    pub(crate) O_con_vec : Vec<String>,
    pub(crate) Name :  String,
    pub(crate) ID : u8,
    pub(crate) content : String,
    pub(crate) frame : egui::epaint::RectShape,
    pub(crate) IO_anker_template : egui::epaint::CircleShape,
    pub(crate) isStart : bool,
}
#[derive(Debug, Clone,PartialEq,)]
pub(crate) struct IO{
    pub(crate) IOVec : Vec<u8>,
    pub(crate) Type : IoType
}
#[derive(Debug, Clone, PartialEq )]
pub(crate) enum IoType {
    Input,
    Output
}
impl State {
    pub(crate) fn new(n_In: usize, n_Out: usize, state_Name: String, Content: String, state_ID: u8, Start_state: bool) -> Self {
        Self {
            O: IO {
                IOVec: vec![0; n_Out],
                Type: IoType::Output,
            },
            I: IO {
                IOVec: vec![0; n_In],
                Type: IoType::Input,
            },
            Name: state_Name,
            ID: state_ID,
            content: Content,
            frame: egui::epaint::RectShape {
                rect: egui::Rect { min: egui::Pos2 { x: 40.0, y: 40.0 }, max: egui::Pos2 { x: 300.0, y: 200.0 } },
                rounding: egui::Rounding { nw: 0.2, ne: 0.2, sw: 0.2, se: 0.2 },
                fill: egui::Color32::from_rgb(96, 96, 96),
                stroke: egui::Stroke::NONE
            },
            IO_anker_template: egui::epaint::CircleShape {
                center: egui::Pos2 { x: 44.0, y: 46.0 },
                radius: 3.0,
                fill: egui::Color32::from_rgb(96, 96, 96),
                stroke: egui::Stroke {
                    width: 1.0,
                    color: egui::Color32::from_rgb(220, 220, 220)
                }
            },
            isStart: Start_state,
            O_con_vec: vec![String::from(""); n_Out],
        }
    }
}