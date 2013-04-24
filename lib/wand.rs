mod types;
mod image;
mod wand_extern;

pub struct MagickWand {
	priv wand_ptr: types::MagickWandPtr
}

//For ExportPixels; represents a single pixel in memory
pub struct RGB(u8, u8, u8);
pub impl RGB {
	fn red(&self) -> u8 {
		let RGB(r, _, _) = *self;
		r
	}
	fn green(&self) -> u8 {
		let RGB(_, g, _) = *self;
		g
	}
	fn blue(&self) -> u8 {
		let RGB(_, _, b) = *self;
		b
	}
}
impl Eq for RGB {
	fn eq(&self, &other: &RGB) -> bool {
		let RGB(r0, g0, b0) = *self;
		let RGB(r1, g1, b1) = other;
		r0 == r1 && g0 == g1 && b0 == b1
	}
	fn ne(&self, &other: &RGB) -> bool {
		return !(self == &other);
	}
}

pub impl MagickWand {
	fn new() -> MagickWand {
		unsafe { MagickWand {wand_ptr: wand_extern::wand::NewMagickWand()} }
	}
	fn isMagickWand(&self) -> bool {
		unsafe { wand_extern::wand::IsMagickWand(self.wand_ptr) }
	}
	fn clear(&self) {
		unsafe { wand_extern::wand::ClearMagickWand(self.wand_ptr) }
	}
	fn adaptiveResizeImage(&self, cols: u32, rows: u32) -> bool {
		unsafe { wand_extern::wand::MagickAdaptiveResizeImage(self.wand_ptr, cols, rows) }
	}
	fn readImage(&self, fname: &str) -> bool {
		let path_bytes = str::to_bytes(fname);

		unsafe {
			let raw_path_bytes = vec::raw::to_ptr(path_bytes);
			wand_extern::wand::MagickReadImage(self.wand_ptr, raw_path_bytes as *i8)
		}
	}
	fn writeImage(&self, fname: &str) -> bool {
		let path_bytes = str::to_bytes(fname);

		unsafe {
			let raw_path_bytes = vec::raw::to_ptr(path_bytes);
			wand_extern::wand::MagickWriteImage(self.wand_ptr, raw_path_bytes as *i8)
		}
	}
	fn readImageBlob(&self, blob: &[u8]) -> bool {
		unsafe {
			wand_extern::wand::MagickReadImageBlob(
			  self.wand_ptr,
			  vec::raw::to_ptr(blob),
			  blob.len() as u32)
		}
	}
	fn getImageBlob(&self) -> ~[u8] {
		let mut len: u32 = 0;
		unsafe {
			let blob = wand_extern::wand::MagickGetImageBlob(self.wand_ptr, &len);
			let v: ~[u8] = vec::from_buf(blob as *u8, len as uint);
			wand_extern::wand::MagickRelinquishMemory(blob);
			return v;
		}
	}
	fn resizeImage(
	  &self,
	  cols: u32,
	  rows: u32,
	  filter: self::types::FilterTypes,
	  blur: f64) -> bool {
		unsafe {
			wand_extern::wand::MagickResizeImage(
			  self.wand_ptr,
			  cols,
			  rows,
			  filter,
			  blur)
		}
	}
	fn imageWidth(&self) -> u32 {
		unsafe {
			wand_extern::wand::MagickGetImageWidth(self.wand_ptr)
		}
	}
	fn imageHeight(&self) -> u32 {
		unsafe {
			wand_extern::wand::MagickGetImageHeight(self.wand_ptr)
		}
	}

	fn exportPixels(&self) -> Option<~[RGB]> {
		//Determine the size of the vector we need to allocate
		let width = self.imageWidth();
		let height = self.imageHeight();
		let num_pixels = (width * height) as uint;
		let mut pixel_buffer = vec::with_capacity::<self::RGB>(num_pixels);
		let mut success: bool;
		unsafe {
			let buffer_ptr = vec::raw::to_ptr(pixel_buffer);
			success = wand_extern::wand::MagickExportImagePixels(
			  self.wand_ptr,
			  0,
			  0,
			  width,
			  height,
			  vec::raw::to_ptr(str::to_bytes("RGB")) as *i8,
			  types::CharPixel,
			  buffer_ptr as *libc::c_void);
			if success {
				vec::raw::set_len::<RGB>(&mut pixel_buffer, num_pixels);
				Some(pixel_buffer)
			} else {
				None
			}
		}
	}
}


impl Drop for MagickWand {
	fn finalize(&self) {
		unsafe { wand_extern::wand::DestroyMagickWand(self.wand_ptr); }
	}
}

// fn exportPixels(&self) -> ~[RGB] {
// 	//Determine the size of the vector we need to allocate
// 	let mut pixel_buffer = vec::with_capacity::<self::RGB>(self.numPixels());
// 	unsafe {
// 		let buffer_ptr = vec::raw::to_ptr(pixel_buffer);
// 		wand_extern::MagickExportImagePixels(buffer_ptr);
// 		vec::raw::set_len::<RGB>(pixel_buffer, self.numPixels());
// 	}
// 	pixel_buffer
// }
// wand.rs:125:28: 125:40 error: mismatched types: expected `&mut ~[RGB]` but found `~[RGB]` (expected &-ptr but found vector)
// wand.rs:125 			vec::raw::set_len::<RGB>(pixel_buffer, num_pixels);
//             			                         ^~~~~~~~~~~~
