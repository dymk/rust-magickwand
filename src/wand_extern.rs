/*
 * Definitions for the C interface of Magick Wand
 */
use types::{FilterTypes, StorageType, MagickWandPtr, PixelWandPtr};
mod types;

//You'll probably need to change this for linux/OSX.
#[link_args = "-LG:\\Programs\\ImageMagick-6.8.4\\VisualMagick\\lib -lCORE_DB_wand_"]
// #[link_args = "-lCORE_DB_wand_"]
pub extern {
	fn MagickWandGenesis();  //Tested
	fn MagickWandTerminus(); //Tested

	fn NewMagickWand() -> MagickWandPtr; //Tested
	// fn NewPixelWand() -> *PixelWand;
	// fn NewPixelWands(number_wands: libc::size_t) -> **PixelWand;

	fn DestroyMagickWand(wand: MagickWandPtr) -> MagickWandPtr; //Tested
	fn ClearMagickWand(wand: MagickWandPtr); //Tested
	fn CloneMagickWand(wand: MagickWandPtr) -> MagickWandPtr; //Tested
	fn MagickRelinquishMemory(resource: *libc::c_void) -> *libc::c_void; //Tested
	fn MagickIdentifyImage(wand: MagickWandPtr) -> *libc::c_char; //Tested
	fn MagickResetIterator(wand: MagickWandPtr); //Tested
	fn IsMagickWand(wand: MagickWandPtr) -> bool; //Tested

	fn MagickGetNumberImages(wand: MagickWandPtr) -> libc::size_t; //Tested
	fn MagickGetImageTotalInkDensity(wand: MagickWandPtr) -> libc::c_double;
	fn MagickHasNextImage(wand: MagickWandPtr) -> bool;
	fn MagickHasPreviousImage(wand: MagickWandPtr) -> bool;

	//Image manipulation functions
	fn MagickNewImage(
	  wand: MagickWandPtr,
	  cols: libc::size_t,
	  rows: libc::size_t,
	  pw: PixelWandPtr) -> bool;
	fn MagickSetImageFormat(wand: MagickWandPtr, format: *libc::c_char) -> bool;
	fn MagickAdaptiveResizeImage(
	  wand: MagickWandPtr,
	  cols: libc::size_t,
	  rows: libc::size_t) -> bool;
	fn MagickResizeImage(
	  wand: MagickWandPtr,
	  cols: libc::size_t,
	  rows: libc::size_t,
	  filter: FilterTypes,
	  blur: libc::c_double) -> bool;
	fn MagickGetImageWidth(wand: MagickWandPtr) -> libc::size_t;
	fn MagickGetImageHeight(wand: MagickWandPtr) -> libc::size_t;
	fn MagickExportImagePixels(
	  wand: MagickWandPtr,
	  x: libc::size_t,
	  y: libc::size_t,
	  cols: libc::size_t,
	  rows: libc::size_t,
	  map: *libc::c_char,
	  storage: StorageType,
	  pix_buff: *libc::c_void) -> bool;
	fn MagickImportImagePixels(
	  wand: MagickWandPtr,
	  x: libc::size_t,
	  y: libc::size_t,
	  cols: libc::size_t,
	  rows: libc::size_t,
	  map: *libc::c_char,
	  storage: StorageType,
	  pix_buff: *libc::c_void) -> bool;

	//Read/write file functions
	// MagickReadImageFile
	fn MagickReadImageBlob(
	  wand: MagickWandPtr,
	  blob: *libc::c_uchar,
	  length: libc::size_t) -> bool; //Tested
	fn MagickReadImage(
	  wand: MagickWandPtr,
	  fname: *libc::c_char) -> bool; //Tested
	fn MagickGetImageBlob(
	  wand: MagickWandPtr,
	  length: *mut libc::size_t) -> *libc::c_void; //Tested
	fn MagickWriteImage(
	  wand: MagickWandPtr,
	  fname: *libc::c_char) -> bool;

	//Pixel wand methods
	fn NewPixelWand() -> PixelWandPtr;
	fn DestroyPixelWand(pw: PixelWandPtr);
	fn IsPixelWand(pw: PixelWandPtr) -> bool;
}
