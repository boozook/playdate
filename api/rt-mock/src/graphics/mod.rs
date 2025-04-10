use crate::ffi::*;


// mod tilemap;
// mod video;
// mod videostream;


pub static GRAPHICS: PlaydateGraphics = PlaydateGraphics {
	// video: &VIDEO,
	clear,
  //  setBackgroundColor: None,
  //  setStencil: None,
	setDrawMode,
  //  setDrawOffset: None,
  //  setClipRect: None,
  //  clearClipRect: None,
  //  setLineCapStyle: None,
  //  setFont: None,
  //  setTextTracking: None,
  //  pushContext: None,
  //  popContext: None,
  //  drawBitmap: None,
  //  tileBitmap: None,
  //  drawLine: None,
  //  fillTriangle: None,
  //  drawRect: None,
	fillRect,
  //  drawEllipse: None,
  //  fillEllipse: None,
  //  drawScaledBitmap: None,
	drawText,
  //  newBitmap: None,
  //  freeBitmap: None,
  //  loadBitmap: None,
  //  copyBitmap: None,
  //  loadIntoBitmap: None,
  //  getBitmapData: None,
  //  clearBitmap: None,
  //  rotatedBitmap: None,
  //  newBitmapTable: None,
  //  freeBitmapTable: None,
  //  loadBitmapTable: None,
  //  loadIntoBitmapTable: None,
  //  getTableBitmap: None,
  //  loadFont: None,
  //  getFontPage: None,
  //  getPageGlyph: None,
  //  getGlyphKerning: None,
	getTextWidth,
  //  getFrame: None,
  //  getDisplayFrame: None,
	getDebugBitmap:
	  if cfg!(feature = "sim") {
		  // TODO:!
		  None
	  } else {
		  None
	  },
  //  copyFrameBufferBitmap: None,
  //  markUpdatedRows: None,
  //  display: None,
  //  setColorToPattern: None,
  //  checkMaskCollision: None,
  //  setScreenClipRect: None,
  //  fillPolygon: None,
  //  getFontHeight: None,
  //  getDisplayBufferBitmap: None,
  //  drawRotatedBitmap: None,
  //  setTextLeading: None,
  //  setBitmapMask: None,
  //  getBitmapMask: None,
  //  setStencilImage: None,
  //  makeFontFromData: None,
  //  getTextTracking: None,
  //  setPixel: None,
  //  getBitmapPixel: None,
  //  getBitmapTableInfo: None,
  //  drawTextInRect: None,
  //  getTextHeightForMaxWidth: None,
  //  drawRoundRect: None,
  //  fillRoundRect: None,
  //  tilemap: &TILEMAP,
  //  videostream: &VIDEOSTREAM
  };

unsafe extern "C" fn clear(color: Color) {}
unsafe extern "C" fn setDrawMode(mode: BitmapDrawMode) -> BitmapDrawMode { mode }
unsafe extern "C" fn drawText(text: *const core::ffi::c_void,
                              len: usize,
                              encoding: StringEncoding,
                              x: core::ffi::c_int,
                              y: core::ffi::c_int)
                              -> core::ffi::c_int {
	0
}
unsafe extern "C" fn fillRect(x: core::ffi::c_int,
                              y: core::ffi::c_int,
                              width: core::ffi::c_int,
                              height: core::ffi::c_int,
                              color: Color) {
}
unsafe extern "C" fn getTextWidth(font: *mut Font,
                                  text: *const core::ffi::c_void,
                                  len: usize,
                                  encoding: StringEncoding,
                                  tracking: core::ffi::c_int)
                                  -> core::ffi::c_int {
	0
}
