use core::ffi::*;
use crate::ffi::*;


pub static DISPLAY: PlaydateDisplay = PlaydateDisplay { getWidth,
                                                        getHeight,
                                                        setRefreshRate,
                                                        setInverted,
                                                        setScale,
                                                        setMosaic,
                                                        setFlipped,
                                                        setOffset,
                                                        getRefreshRate,
                                                        getFPS };


unsafe extern "C" fn getWidth() -> c_int { LCD_COLUMNS as _ }
unsafe extern "C" fn getHeight() -> c_int { LCD_ROWS as _ }
unsafe extern "C" fn setInverted(rate: c_int) {}
unsafe extern "C" fn setScale(rate: c_uint) {}
unsafe extern "C" fn setMosaic(x: c_uint, y: c_uint) {}
unsafe extern "C" fn setFlipped(x: c_int, y: c_int) {}
unsafe extern "C" fn setOffset(x: c_int, y: c_int) {}


/// nominal display refresh rate
static mut FPS: c_float = 0.0;

unsafe extern "C" fn setRefreshRate(rate: c_float) { FPS = rate }
unsafe extern "C" fn getRefreshRate() -> c_float { FPS }
unsafe extern "C" fn getFPS() -> c_float { if FPS == 0. { 30. } else { FPS } }
