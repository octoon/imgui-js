
use stdweb::unstable::TryInto;


#[derive(Debug, Clone)]
pub struct ImguiIO{
    imgui_io:stdweb::Value
}

#[derive(Debug, Clone)]
pub struct ImDrawData{
    imgui_draw_data:stdweb::Value
}

impl ImDrawData{
    pub fn iterate_draw_lists(&self, callback:fn(stdweb::Value)->()){
        js! {
            window.Imgui.IterateDrawLists(@{callback});
        };
    }
}

#[derive(Debug, Clone)]
pub struct ImDrawList{
    draw_list:stdweb::Value
}

impl ImDrawList{
    pub fn vtx_buffer(&self) -> stdweb::Value{
        let list = &self.draw_list;
        let ret = js!{return @{list}.VtxBuffer;};
        ret
    }

    pub fn idx_buffer(&self) -> stdweb::Value{
        let list = &self.draw_list;
        let ret = js!{return @{list}.IdxBuffer;};
        ret
    }

    pub fn flags(&self) -> stdweb::Value{
        let list = &self.draw_list;
        let ret = js!{return @{list}.Flags;};
        ret
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
        ret.try_into().unwrap()
    }

    #[allow(non_snake_case)]
    pub fn IMGUI_CHECKVERSION(&self){
        js! {
            window.Imgui.IMGUI_CHECKVERSION();
        };
    }

    pub fn create_context(&self){
        js! {
            return window.Imgui.CreateContext();
        };
    }

    pub fn new_frame(&self){
        js! {
            return window.Imgui.NewFrame();
        };
    }

    pub fn end_frame(&self){
        js! {
            return window.Imgui.EndFrame();
        };
    }

    pub fn render(&self){
        js! {
            return window.Imgui.Render();
        };
    }

    pub fn get_io(&self) -> ImguiIO{
        let ret = js! {
            return window.Imgui.GetIO();
        };
        ImguiIO{
            imgui_io:ret
        }
    }

    pub fn get_draw_data(&self) -> ImDrawData{
        let ret = js! {
            return window.Imgui.GetDrawData();
        };
        ImDrawData{
            imgui_draw_data:ret
        }
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