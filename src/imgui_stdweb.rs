


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
}