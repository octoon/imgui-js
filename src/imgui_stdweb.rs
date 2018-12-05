

#[derive(Debug, Clone)]
pub struct ImguiIO{
}

#[derive(Debug, Clone)]
pub struct ImguiDrawData{
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

    pub fn iterate_draw_lists(&self, callback:fn(stdweb::Value)->()){
        js! {
            window.Imgui.IterateDrawLists(@{callback});
        };
    }

    pub fn iterate_draw_cmds(&self, callback:fn(stdweb::Value)->()){
        js! {
            window.Imgui.IterateDrawCmds(@{callback});
        };
    }
}

#[derive(Debug, Clone)]
pub struct Imgui{
}


impl Imgui{
    pub fn new() -> Imgui{
        Imgui{
        }
    }

    #[allow(non_snake_case)]
    pub fn IMGUI_VERSION(&self) -> String{
        let ret = js! {
            return window.Imgui.IMGUI_VERSION;
        };
        ret
    }

    #[allow(non_snake_case)]
    pub fn IMGUI_CHECKVERSION(&self){
        js! {
            window.Imgui.IMGUI_CHECKVERSION();
        };
    }

    pub fn create_context(&self){
        js! {
            return window.Imgui.create_context();
        };
    }

    pub fn get_io(&self) -> stdweb::Value{
        let ret = js! {
            return window.Imgui.get_io();
        };
        ret
    }

    pub fn get_draw_data(&self) -> stdweb::Value{
        let ret = js! {
            return window.Imgui.GetDrawData();
        };
        ret
    }

    pub fn style_colors_dark(&self){
        js! {
            return window.Imgui.StyleColorsDark();
        };
    }

    #[allow(non_snake_case)]
    pub fn IM_ASSERT(&self, cond:bool){
        js! {
            window.Imgui.IM_ASSERT(@{cond});
        };
    }
}