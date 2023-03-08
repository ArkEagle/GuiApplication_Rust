use eframe::egui::{Options, Vec2};

struct state {
    IOs : [IO; 2],
    Name : &'static str,
    ID : u8,
    content :&'static str
}
struct IO{
    IOVec : Vec<u8>,
    Type : IO_type
}

enum IO_type {
    Input,
    Output
}
impl state {
    fn default(n_In:usize,n_Out:usize,state_Name:String,Content:String)->Self{
        let I : IO = IO{
            IOVec : Vec::with_capacity(n_In),
            Type : IO_type::Input,
        };
        let O : IO = IO{
            IOVec : Vec::with_capacity(n_Out),
            Type : IO_type::Output,
        };
        Self{
            IOs : [I,O],
            Name : state_Name.into(),
            ID : u8,
            content : Content.into()
        }
    }
}