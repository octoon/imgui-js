

#[derive(Debug, Clone)]
pub struct ImguiIO{
}

#[derive(Debug, Clone)]
pub struct ImDrawList{
    draw_list:stdweb::Value
}

impl ImDrawList{
    pub fn vtx_buffer(&self) -> &stdweb::Value{
        self.draw_list.VtxBuffer
    }

    pub fn idx_buffer(&self) -> &stdweb::Value{
        self.draw_list.IdxBuffer
    }

    pub fn flags(&self) -> &stdweb::Value{
        self.draw_list.Flags
    }

    pub fn iterate_draw_lists(&self){
        
    }
}

#[derive(Debug, Clone)]
pub struct Imgui{
    module: stdweb::Value
}


impl Imgui{
    pub fn new() -> Imgui{
        Imgui{
            module:js!{return window.Imgui;}
        }
    }

    #[allow(non_snake_case)]
    pub fn IMGUI_CHECKVERSION(&self){
        let module = &self.module;
        js! {
            @{module}.IMGUI_CHECKVERSION();
        };
    }

    pub fn create_context(&self){
        let module = &self.module;
        js! {
            return @{module}.create_context();
        };
    }

    pub fn get_io(&self) -> stdweb::Value{
        let module = &self.module;
        let ret = js! {
            return @{module}.get_io();
        };
        ret
    }

    pub fn style_colors_dark(&self){
        let module = &self.module;
        js! {
            return @{module}.StyleColorsDark();
        };
    }

    #[allow(non_snake_case)]
    pub fn IM_ASSERT(&self, cond:bool){
        let module = &self.module;
        js! {
            @{module}.IM_ASSERT(@{cond});
        };
    }
}