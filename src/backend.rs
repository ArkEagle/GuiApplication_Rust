use eframe::egui;
use eframe::egui::{Align, Galley, Ui};
use std::{fs,path,io};
use eframe::epaint::tessellator::Path;
use serde::{Serialize,Deserialize};
#[derive(Debug, Clone, PartialEq,Serialize, Deserialize)]
pub(crate) struct State {
    pub(crate) ID : u8,
    pub(crate) Name :  String,
    pub(crate) isStart : bool,
    pub(crate) I : IO,
    pub(crate) O : IO,
    pub(crate) O_con_vec : Vec<String>,
    pub(crate) content : String,
    pub(crate) frame : egui::epaint::RectShape,
    pub(crate) IO_anker_template : egui::epaint::CircleShape,
    pub(crate) current_scale : f32,
}
#[derive(Debug, Clone,PartialEq,Serialize, Deserialize)]
pub(crate) struct IO{
    pub(crate) IOVec : Vec<u8>,
    pub(crate) Type : IoType
}
#[derive(Debug, Clone, PartialEq,Serialize, Deserialize )]
pub(crate) enum IoType {
    Input,
    Output
}
impl State {

    pub(crate) fn new(n_In:usize, n_Out:usize, state_Name:String, Content: String, state_ID:u8,Start_state:bool, scale : f32) ->Self{

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
                rect: egui::Rect{min: egui::Pos2 { x:40.0*scale, y :40.0*scale },max: egui::Pos2{ x:300.0*scale, y :200.0*scale }},
                rounding: egui::Rounding{nw:2.0,ne:2.0,sw:2.0,se:2.0},
                fill: egui::Color32::from_rgb(96, 96, 96),
                stroke: egui::Stroke::NONE
            },
            IO_anker_template : egui::epaint::CircleShape{
                center: egui::Pos2{x:44.0, y :46.0},
                radius:3.0,
                fill: egui::Color32::from_rgb(96, 96, 96),
                stroke:egui::Stroke{width: 1.0,color: egui::Color32::from_rgb(220, 220, 220)
                }},
            isStart : Start_state,
            O_con_vec : vec![String::from("");n_Out],
            current_scale : scale,
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
    /// Output ungleich None, wenn ein IO gedrückt wurde
    pub(crate) fn Draw_Box(&mut self, ui: &mut egui::Ui, scale : f32, scrollDelta : egui::Vec2){
        self.frame.rect.min.x+=scrollDelta.x;
        self.frame.rect.min.y+=scrollDelta.y;
        self.frame.rect.max.x+=scrollDelta.x;
        self.frame.rect.max.y+=scrollDelta.y;
        if self.current_scale != scale{
            let zoom = scale/self.current_scale;
            self.frame.rect.min.x *=  zoom;
            self.frame.rect.min.y *=  zoom;
            self.frame.rect.max.x *=  zoom;
            self.frame.rect.max.y *=  zoom;
            self.frame.rounding.se *= zoom;
            self.frame.rounding.ne *= zoom;
            self.frame.rounding.sw *= zoom;
            self.frame.rounding.nw *= zoom;
            self.current_scale = scale;
        }
        ui.painter().add(self.frame);
    }
    pub(crate) fn Draw_IO(&mut self, ui: &mut egui::Ui, scale : &f32 ) -> Option<clickedIO>{
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
            anker.center.x = self.frame.rect.min.x+4.0*scale;
            anker.center.y = self.frame.rect.min.y+(offset as f32*10.0+4.0)*scale;
            //anker.center.y *= scale;
            anker.radius *= scale;
            ui.painter().add(anker);
            let r_a_i = ui.allocate_rect(anker.visual_bounding_rect(),egui::Sense::click_and_drag());
            if r_a_i.clicked(){
                //*i = 2;
                clicked_IO = Option::from(clickedIO{
                    IOType : IoType::Input,
                    IO_number : offset,
                    State : self.ID,
                });
            }
            offset += 1;
            leng_I = (offset as f32*10.0+6.0)*scale;
        }
        //==============Drawing Outputs==============
        offset = 0;
        for (n,o) in self.O.IOVec.iter_mut().enumerate(){
            let mut anker = self.IO_anker_template.clone();
            if *o != 0 as u8 {
                anker.fill = egui::Color32::from_rgb(255, 255, 255);
            }
            anker.center.x = self.frame.rect.max.x-4.0*scale;
            anker.center.y = self.frame.rect.min.y+(offset as f32*10.0+4.0)*scale;;
            anker.radius *= scale;
            ui.painter().add(anker);
            let r_a_o = ui.allocate_rect(anker.visual_bounding_rect(),egui::Sense::click_and_drag());
            if r_a_o.clicked(){
                //*o = 2;
                clicked_IO = Option::from(clickedIO{
                    IOType : IoType::Output,
                    IO_number : offset,
                    State : self.ID,
                });
            }
            r_a_o.on_hover_text(&self.O_con_vec[n]);


            offset += 1;
            leng_O = (offset as f32*10.0+6.0)*scale;
        }
        //==============Resize Rect==============
        if self.frame.rect.max.y-self.frame.rect.min.y < leng_I {
            self.frame.rect.max.y = leng_I;
        }
        else if  self.frame.rect.max.y-self.frame.rect.min.y < leng_O{
            self.frame.rect.max.y = leng_O;
        }
        return clicked_IO;
    }
    pub(crate) fn DrawTitle(&self, ui: &mut egui::Ui, scale : f32, ){
        let TitleRect : egui::Rect  = egui::Rect{
            min : egui::Pos2{x:self.frame.rect.min.x+6.0*scale, y:self.frame.rect.min.y+6.0*scale},
            max : egui::Pos2{x:self.frame.rect.max.x-6.0*scale, y:self.frame.rect.min.y+26.0*scale}
        };
        let mut TitleText : egui::widgets::Label = egui::widgets::Label::new(egui::RichText::from(self.Name.clone()).size(12.0*scale));
        if self.isStart{ 
            TitleText = egui::widgets::Label::new(egui::RichText::from(String::from("STARTSTATE \n")+&self.Name.clone()).size(12.0*scale));
        }

        ui.put(TitleRect,TitleText);

    }
    pub(crate) fn DrawContent(&mut self, ui : &mut egui::Ui, scale : f32, ) {
        let ContentRect : egui::Rect  = egui::Rect{
            min : egui::Pos2{x:self.frame.rect.min.x+15.0*scale, y:self.frame.rect.min.y+30.0*scale},
            max : egui::Pos2{x:self.frame.rect.max.x-15.0*scale, y:self.frame.rect.max.y-5.0*scale}};
        let size = egui::Vec2{ x: ContentRect.max.x-ContentRect.min.x, y: ContentRect.max.y-ContentRect.min.y };
        let ui_offset : f32 = 5.0;
        let UiRect : egui::Rect  = egui::Rect{
            min : egui::Pos2{x:self.frame.rect.min.x+(15.0+ui_offset)*scale, y:self.frame.rect.min.y+(30.0+ui_offset)*scale},
            max : egui::Pos2{x:self.frame.rect.max.x-(15.0+ui_offset)*scale, y:self.frame.rect.max.y-(5.0+ui_offset)*scale}};

        let mut  childui = ui.child_ui(UiRect,egui::Layout::left_to_right(egui::Align::TOP));
        let textbox =  egui::epaint::RectShape{
            rect: ContentRect,
            rounding: egui::Rounding{nw:6.0*scale,ne:6.0*scale,sw:6.0*scale,se:6.0*scale},
            fill: egui::Color32::from_rgb(0, 0, 0),
            stroke: Default::default(),
        };

        let mut ContentScroll = egui::ScrollArea::both().id_source(self.ID).auto_shrink([false;2]).show(&mut childui,|ui|{
            ui.painter().add(textbox);
            ui.label(egui::RichText::from(&self.content).size(11.0*scale));
        });
        ContentScroll.inner_rect = ContentRect;
        }

    pub(crate) fn refactorState(&mut self,n_In:usize, n_Out:usize, state_Name:String, Content: String,Start_state:bool){
        self.O.IOVec = vec![0;n_Out];
        self.I.IOVec = vec![0;n_In];
        self.Name = state_Name;
        self.content = Content;
        self.isStart = Start_state;
        self.O_con_vec = vec![String::from("");n_Out];
    }
}
#[derive(Debug, Clone, PartialEq,Serialize, Deserialize )]
pub(crate) struct clickedIO{
    pub(crate) IOType : IoType,
    pub(crate) IO_number : u8,
    pub(crate) State : u8,
}
