#[doc(hidden)]
#[no_mangle]
pub extern "C" fn fminf(x: f32, y: f32) -> f32 {
    libm::fminf(x, y)
}

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn fmaxf(x: f32, y: f32) -> f32 {
    libm::fmaxf(x, y)
}

use rusttype::Font;
pub fn init() -> Option<Font<'static>> {
    let system_font: &[u8; 171656]
        = include_bytes!("../../../fonts/Roboto/Roboto-Medium.ttf");
    Font::try_from_bytes(system_font)
}
