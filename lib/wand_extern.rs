/*
 * Definitions for the C interface of Magick Wand
 */

mod types;

//You'll probably need to change this for linux/OSX.
#[link_name = "CORE_DB_wand_"]
#[link_args = "-LG:\\Programs\\ImageMagick-6.8.4\\VisualMagick\\lib"]
pub extern mod wand {

	fn MagickWandGenesis();  //Tested
	fn MagickWandTerminus(); //Tested

	fn NewMagickWand() -> types::MagickWandPtr; //Tested
	// fn NewPixelWand() -> *PixelWand;
	// fn NewPixelWands(number_wands: libc::size_t) -> **PixelWand;

	fn DestroyMagickWand(wand: types::MagickWandPtr) -> types::MagickWandPtr; //Tested
	fn ClearMagickWand(wand: types::MagickWandPtr); //Tested
	fn MagickRelinquishMemory(resource: *libc::c_void) -> *libc::c_void; //Tested
	fn MagickIdentifyImage(wand: types::MagickWandPtr) -> *libc::c_char; //Tested
	fn MagickResetIterator(wand: types::MagickWandPtr); //Tested
	fn IsMagickWand(wand: types::MagickWandPtr) -> bool; //Tested

	//Image manipulation functions
	fn MagickSetImageFormat(wand: types::MagickWandPtr, format: *libc::c_char) -> bool;
	fn MagickAdaptiveResizeImage(
	  wand: types::MagickWandPtr,
	  cols: libc::size_t,
	  rows: libc::size_t) -> bool;
	fn MagickResizeImage(
	  wand: types::MagickWandPtr,
	  cols: libc::size_t,
	  rows: libc::size_t,
	  filter: super::types::FilterTypes,
	  blur: libc::c_double) -> bool;
	fn MagickGetImageWidth(wand: types::MagickWandPtr) -> libc::size_t;
	fn MagickGetImageHeight(wand: types::MagickWandPtr) -> libc::size_t;
	fn MagickExportImagePixels(
	  wand: types::MagickWandPtr,
	  x: libc::size_t,
	  y: libc::size_t,
	  cols: libc::size_t,
	  rows: libc::size_t,
	  map: *libc::c_char,
	  storage: super::types::StorageType,
	  pix_buff: *libc::c_void) -> bool;

	//Read/write file functions
	// MagickReadImageFile
	fn MagickReadImageBlob(
	  wand: types::MagickWandPtr,
	  blob: *libc::c_uchar,
	  length: libc::size_t) -> bool; //Tested
	fn MagickReadImage(
	  wand: types::MagickWandPtr,
	  fname: *libc::c_char) -> bool; //Tested
	fn MagickGetImageBlob(
	  wand: types::MagickWandPtr,
	  length: *libc::size_t) -> *libc::c_void; //Tested
	fn MagickWriteImage(
	  wand: types::MagickWandPtr,
	  fname: *libc::c_char) -> bool;
}
