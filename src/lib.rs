#[macro_use]
#[cfg(all(target_arch = "wasm32"))]
extern crate stdweb;

#[cfg(not(target_arch = "wasm32"))]
pub struct Imgui{
}

#[cfg(not(target_arch = "wasm32"))]
impl Imgui{
    pub fn new() -> Imgui{
        Imgui{
        }
    }

    #[allow(non_snake_case)]
    pub fn IMGUI_CHECKVERSION(&self){
    }
}

#[cfg(all(target_arch = "wasm32"))]
pub struct Imgui{
    module: stdweb::Value
}

#[cfg(all(target_arch = "wasm32"))]
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
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
