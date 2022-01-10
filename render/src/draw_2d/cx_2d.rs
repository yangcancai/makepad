use {
    std::{
        ops::Deref,
        ops::DerefMut
    },
    makepad_shader_compiler::makepad_math::{
        Vec2
    },
    crate::{
        event::{
            DrawEvent
        },
        area::{
            Area,
        },
        draw_2d::{
            view::View,
            turtle::Turtle,
        },
        pass::{
            Pass,
            CxPassDepOf
        },
        cx::Cx,
    }
};

pub struct Cx2d<'a> {
    pub cx: &'a mut Cx,
    pub draw_event: &'a DrawEvent,
    pub pass_id: Option<usize>,
    pub draw_list_stack: Vec<usize>,
    pub turtles: Vec<Turtle>,
    pub align_list: Vec<Area>,
    pub current_dpi_factor: f32,
}

impl<'a> Deref for Cx2d<'a> {type Target = Cx; fn deref(&self) -> &Self::Target {self.cx}}
impl<'a> DerefMut for Cx2d<'a> {fn deref_mut(&mut self) -> &mut Self::Target {self.cx}}

impl<'a> Cx2d<'a> {
    pub fn new(cx: &'a mut Cx, draw_event:&'a DrawEvent) -> Self {
        Self {
            current_dpi_factor: cx.default_dpi_factor,
            cx: cx,
            draw_event,
            pass_id: None,
            draw_list_stack: Vec::new(),
            turtles: Vec::new(),
            align_list: Vec::new(),
        }
    }
    
    pub fn begin_pass(&mut self, pass:&Pass) {
        if self.pass_id.is_some(){panic!()}
        
        self.pass_id = Some(pass.pass_id);
        let cxpass = &mut self.passes[pass.pass_id];

        cxpass.main_draw_list_id = None;

        match cxpass.dep_of{
            CxPassDepOf::Window(window_id)=>{
                self.passes[pass.pass_id].pass_size = self.windows[window_id].get_inner_size();
                self.current_dpi_factor = self.get_delegated_dpi_factor(pass.pass_id);
            }
            CxPassDepOf::Pass(pass_id)=>{
                self.passes[pass.pass_id].pass_size = self.passes[pass_id].pass_size;
                self.current_dpi_factor = self.get_delegated_dpi_factor(pass_id);
            }
            _=>{
                cxpass.override_dpi_factor = Some(1.0);
                self.current_dpi_factor = 1.0;
            }
        }
    }
    
    pub fn end_pass(&mut self, pass:&Pass){
        if self.pass_id != Some(pass.pass_id){
            panic!();
        }
        self.pass_id = None;
        if self.draw_list_stack.len()>0 {
            panic!("Draw list stack disaligned, forgot an end_view(cx)");
        }
        if self.turtles.len()>0 {
            panic!("Turtle stack disaligned, forgot an end_turtle()");
        }
    }
    
    pub fn get_scroll_pos(&self) -> Vec2 {
        let draw_list = &self.draw_lists[*self.draw_list_stack.last().unwrap()];
        draw_list.unsnapped_scroll
    }

    pub fn view_will_redraw(&self, view: &View) -> bool {
        
        if self.draw_event.redraw_all {
            return true;
        }
        // figure out if areas are in some way a child of view_id, then we need to redraw
        for check_draw_list_id in &self.draw_event.draw_lists {
            let mut next = Some(*check_draw_list_id);
            while let Some(vw) = next{
                if vw == view.draw_list_id {
                    return true
                }
                next = self.draw_lists[vw].codeflow_parent_id;
            }
        }
        // figure out if areas are in some way a parent of view_id, then redraw
        for check_draw_list_id in &self.draw_event.draw_lists_and_children {
            let mut next = Some(view.draw_list_id);
            while let Some(vw) = next{
                if vw == *check_draw_list_id {
                    return true
                }
                next = self.draw_lists[vw].codeflow_parent_id;
            }
        }
        false
    }
}

