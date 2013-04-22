pub mod types;
mod image;
mod wand_extern;

pub struct MagickWand {
	priv wand_ptr: types::MagickWandPtr
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
	  filter: types::FilterTypes,
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
}
impl Drop for MagickWand {
	fn finalize(&self) {
		unsafe { wand_extern::wand::DestroyMagickWand(self.wand_ptr); }
	}
}
