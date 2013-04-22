/*
 * Definitions for the C interface of Magick Wand
 */

mod c_types;

#[link_name = "CORE_DB_wand_"]
#[link_args = "-LG:\\Programs\\ImageMagick-6.8.4\\VisualMagick\\lib"]
pub extern mod wand {

	fn MagickWandGenesis();  //Tested
	fn MagickWandTerminus(); //Tested

	fn NewMagickWand() -> c_types::MagickWandPtr; //Tested
	// fn NewPixelWand() -> *PixelWand;
	// fn NewPixelWands(number_wands: libc::size_t) -> **PixelWand;

	fn DestroyMagickWand(wand: c_types::MagickWandPtr) -> c_types::MagickWandPtr; //Tested
	fn ClearMagickWand(wand: c_types::MagickWandPtr); //Tested
	fn MagickRelinquishMemory(resource: *libc::c_void) -> *libc::c_void; //Tested
	fn MagickIdentifyImage(wand: c_types::MagickWandPtr) -> *libc::c_char; //Tested
	fn MagickResetIterator(wand: c_types::MagickWandPtr); //Tested
	fn IsMagickWand(wand: c_types::MagickWandPtr) -> bool; //Tested

	//Image manipulation functions
	fn MagickSetImageFormat(wand: c_types::MagickWandPtr, format: *libc::c_char) -> bool;
	fn MagickAdaptiveResizeImage(wand: c_types::MagickWandPtr, cols: libc::size_t, rows: libc::size_t) -> bool;

	//Read/write functions
	// MagickReadImageFile
	fn MagickReadImageBlob(wand: c_types::MagickWandPtr, blob: *libc::c_uchar, length: libc::size_t) -> bool; //Tested
	fn MagickReadImage(wand: c_types::MagickWandPtr, fname: *libc::c_char) -> bool; //Tested
	fn MagickGetImageBlob(wand: c_types::MagickWandPtr, length: *libc::size_t) -> *libc::c_void; //Tested
	fn MagickWriteImage(wand: c_types::MagickWandPtr, fname: *libc::c_char) -> bool;
}
