use std::os::raw::c_float;
use std::sync::Arc;
use eframe::egui;
use eframe::egui::{Galley, Ui};
use tracing_subscriber::fmt::format;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct State {
    pub(crate) I : IO,
    pub(crate) O : IO,
    pub(crate) Name :  String,
    pub(crate) ID : u8,
    pub(crate) content : String,
    pub(crate) frame : egui::epaint::RectShape,
    pub(crate) IO_anker_template : egui::epaint::CircleShape,

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

    pub(crate) fn new(n_In:usize, n_Out:usize, state_Name:String, Content: String, state_ID:u8) ->Self{

        Self{
            O : IO{
            IOVec : vec![0;n_Out],
            Type : IoType::Output,
            },
            I : IO{
            IOVec : vec![0;n_In],
            Type : IoType::Input,
            },
            Name : state_Name,
            ID : state_ID,
            content : Content,
            frame : egui::epaint::RectShape{
                rect: egui::Rect{min: egui::Pos2 { x:40.0, y :40.0 },max: egui::Pos2{ x:100.0, y :200.0 }},
                rounding: egui::Rounding{nw:0.2,ne:0.2,sw:0.2,se:0.2},
                fill: egui::Color32::from_rgb(96, 96, 96),
                stroke: egui::Stroke::NONE
            },
            IO_anker_template : egui::epaint::CircleShape{
                center: egui::Pos2{x:44.0, y :46.0},
                radius:3.0,
                fill: egui::Color32::from_rgb(96, 96, 96),
                stroke:egui::Stroke{width: 1.0,color: egui::Color32::from_rgb(220, 220, 220)
                }},

        }

    }
    pub(crate) fn Connect_IO(&mut self,Type : IoType, Pos : usize, State_ID : u8){
        match Type{
            IoType::Input => {
                self.I.IOVec[Pos] = State_ID;
            }
            IoType::Output => {
                self.O.IOVec[Pos] = State_ID;
            }
        }

    }
    pub(crate) fn change_IO_count(&mut self,Type : IoType,IOcount : usize){
        match Type {
            IoType::Input => {
                self.I.IOVec = Vec::new();
                self.I.IOVec = Vec::with_capacity(IOcount);
                for i in 0..self.I.IOVec.len()-1 {
                    self.I.IOVec.push(0);
                }
            },
            IoType::Output => {
                self.O.IOVec = Vec::new();
                self.O.IOVec = Vec::with_capacity(IOcount);
                for i in 0..self.O.IOVec.len()-1 {
                    self.O.IOVec.push(0);
                }
            }
        }
    }
    /// Output ungleich None, wenn ein IO gedr??ckt wurde: (IO-Typ,IO-Number State-ID)
    pub(crate) fn Draw_IO(&mut self, ui: &mut egui::Ui) -> Option<clickedIO>{
        let mut clicked_IO = None;
        let mut leng_I : f32 = 0.0;
        let mut leng_O : f32 = 0.0;
        //==============Drawing Inputs==============
        let mut offset:u8 = 0;
        for i in  self.I.IOVec.iter_mut(){
            let mut anker = self.IO_anker_template.clone();
            if *i != 0 as u8 {
                anker.fill = egui::Color32::from_rgb(255, 255, 255);
            }
            anker.center.y += offset as f32*10.0;
            ui.painter().add(anker);
            let r_a_i = ui.allocate_rect(anker.visual_bounding_rect(),egui::Sense::click_and_drag());
            if r_a_i.clicked(){
                *i = 2;
                clicked_IO = Option::from(clickedIO{
                    IOType : IoType::Input,
                    IO_number : offset,
                    State : self.ID,
                });
            }

            offset += 1;
            leng_I = offset as f32*10.0+6.0;
        }
        //==============Drawing Outputs==============
        offset = 0;
        for o in self.O.IOVec.iter_mut(){
            let mut anker = self.IO_anker_template.clone();
            if *o != 0 as u8 {
                anker.fill = egui::Color32::from_rgb(255, 255, 255);
            }
            anker.center.x = self.frame.rect.max.x-4.0;
            anker.center.y += offset as f32*10.0;
            ui.painter().add(anker);
            let r_a_o = ui.allocate_rect(anker.visual_bounding_rect(),egui::Sense::click_and_drag());
            if r_a_o.clicked(){
                *o = 2;
                clicked_IO = Option::from(clickedIO{
                    IOType : IoType::Output,
                    IO_number : offset,
                    State : self.ID,
                });
            }


            offset += 1;
            leng_O = offset as f32*10.0+6.0;
        }
        //==============Resize Rect==============
        if self.frame.rect.max.y-self.frame.rect.min.y < leng_I {
            self.frame.rect.max.y = leng_I;
        }
        else if  self.frame.rect.max.y-self.frame.rect.min.y < leng_O{
            self.frame.rect.max.y = leng_O;
        }
        return clicked_IO
    }
    pub(crate) fn DrawTitle(&self, ui: &mut egui::Ui,ctx : &egui::Context){
        let TitleRect : egui::Rect  = egui::Rect{
            min : egui::Pos2{x:self.frame.rect.min.x+6.0, y:self.frame.rect.min.y+6.0},
            max : egui::Pos2{x:self.frame.rect.max.x-6.0, y:self.frame.rect.min.y+26.0}
        };
        let mut TitleText : egui::widgets::Label = egui::widgets::Label::new(egui::RichText::from(self.Name.clone()).size(12.0));
        ui.put(TitleRect,TitleText);

    }
}

pub(crate) struct clickedIO{
    pub(crate) IOType : IoType,
    pub(crate) IO_number : u8,
    pub(crate) State : u8,
}