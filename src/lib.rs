#[macro_use]
#[cfg(all(target_arch = "wasm32"))]
extern crate stdweb;

#[cfg(not(target_arch = "wasm32"))]
extern crate imgui_sys;

#[cfg(all(target_arch = "wasm32"))]
pub mod imgui_stdweb;

#[cfg(all(target_arch = "wasm32"))]
pub use self::imgui_stdweb::*;

#[cfg(not(target_arch = "wasm32"))]
pub mod imgui_c;

#[cfg(not(target_arch = "wasm32"))]
pub use self::imgui_c::*;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
