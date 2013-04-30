use types;
use wand_extern;
use types;
use pixel;

pub struct MagickWand {
	priv wand_ptr: types::MagickWandPtr
}

pub impl MagickWand {
	fn new() -> MagickWand {
		let ptr;
		unsafe { ptr = wand_extern::wand::NewMagickWand() }
		MagickWand::new_with_ptr(ptr)
	}
	priv fn new_with_ptr(ptr: types::MagickWandPtr) -> MagickWand {
		MagickWand { wand_ptr: ptr }
	}

	fn is_magick_wand(&self) -> bool {
		unsafe { wand_extern::wand::IsMagickWand(self.wand_ptr) }
	}
	fn clear(&self) {
		unsafe { wand_extern::wand::ClearMagickWand(self.wand_ptr) }
	}
	fn adaptive_resize_image(&self, cols: u32, rows: u32) -> bool {
		unsafe { wand_extern::wand::MagickAdaptiveResizeImage(self.wand_ptr, cols, rows) }
	}
	fn read_image(&self, fname: &str) -> bool {
		let path_bytes = str::to_bytes(fname);

		unsafe {
			let raw_path_bytes = vec::raw::to_ptr(path_bytes);
			wand_extern::wand::MagickReadImage(self.wand_ptr, raw_path_bytes as *i8)
		}
	}
	fn write_image(&self, fname: &str) -> bool {
		let path_bytes = str::to_bytes(fname);

		unsafe {
			let raw_path_bytes = vec::raw::to_ptr(path_bytes);
			wand_extern::wand::MagickWriteImage(self.wand_ptr, raw_path_bytes as *i8)
		}
	}
	fn read_imageBlob(&self, blob: &[u8]) -> bool {
		unsafe {
			wand_extern::wand::MagickReadImageBlob(
			  self.wand_ptr,
			  vec::raw::to_ptr(blob),
			  blob.len() as u32)
		}
	}
	fn image_blob(&self) -> ~[u8] {
		let mut len: u32 = 0;
		unsafe {
			let blob = wand_extern::wand::MagickGetImageBlob(self.wand_ptr, &len);
			let v: ~[u8] = vec::from_buf(blob as *u8, len as uint);
			wand_extern::wand::MagickRelinquishMemory(blob);
			return v;
		}
	}
	fn resize_image(
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
	fn image_width(&self) -> uint {
		unsafe {
			wand_extern::wand::MagickGetImageWidth(self.wand_ptr) as uint
		}
	}
	fn image_height(&self) -> uint {
		unsafe {
			wand_extern::wand::MagickGetImageHeight(self.wand_ptr) as uint
		}
	}

	fn export_pixels_flat<T: pixel::FromRGB + Copy>(&self) -> Option<~[T]> {
		let width = self.image_width();
		let height = self.image_height();
		let num_pixels = (width * height);
		let mut pixel_buffer = vec::with_capacity::<pixel::RGB>(num_pixels);

		unsafe {
			let buffer_ptr = vec::raw::to_ptr(pixel_buffer);
			let success = wand_extern::wand::MagickExportImagePixels(
			  self.wand_ptr,
			  0,
			  0,
			  width as u32,
			  height as u32,
			  vec::raw::to_ptr(str::to_bytes("RGB")) as *i8,
			  types::CharPixel,
			  buffer_ptr as *libc::c_void);
			if success {
				vec::raw::set_len::<pixel::RGB>(&mut pixel_buffer, num_pixels);

				//Map to the requested pixel type
				let pixel_buffer: ~[T] = do pixel_buffer.map |p| {
					pixel::FromRGB::from_rgb(*p)
				};

				Some(pixel_buffer)
			} else {
				None
			}
		}
	}

	fn export_pixels<T : pixel::FromRGB + Copy>(&self) -> Option<~[~[T]]> {

		//Determine the size of the vector we need to allocate
		let width = self.image_width();
		let height = self.image_height();

		let flat_pixels  = match self.export_pixels_flat() {
			Some(p) => p,
			None    => return None
		};

		//Make it a nested array of pixels
		let mut mapped_pixel_buffer = vec::with_capacity::<~[T]>(height);
		for uint::range(0, height) |h| {
			let row = flat_pixels.slice(h * width, (h+1) * width).to_owned();
			mapped_pixel_buffer.push(row);
		}
		Some(mapped_pixel_buffer)
	}

	fn import_pixels_flat<T : pixel::ToRGB>(
	    &self,
	    width: uint,
	    height: uint,
	    pixel_buffer: &[T]) -> bool {
		assert!(width * height == pixel_buffer.len());

		let rgb_pixel_buffer = pixel_buffer.map(|p| {
			p.to_rgb()
		});

		let success;
		unsafe {
			let rgb_buffer_ptr = vec::raw::to_ptr(rgb_pixel_buffer);
			success = wand_extern::wand::MagickImportImagePixels(
			  self.wand_ptr,
			  0, 0, width as u32, height as u32,
			  vec::raw::to_ptr(str::to_bytes("RGB")) as *i8,
			  types::CharPixel,
			  rgb_buffer_ptr as *libc::c_void);
		}

		return success;
	}

	fn import_pixels<T : pixel::ToRGB + Copy>(
	    &self,
	    pixel_buffer: &[~[T]]) -> bool {
		let height = pixel_buffer.len();
		let flat_pixels = vec::concat(pixel_buffer);
		let width = flat_pixels.len() / pixel_buffer.len();
		return self.import_pixels_flat::<T>(width, height, flat_pixels)
	}

	fn num_images(&self) -> u32 {
		unsafe { wand_extern::wand::MagickGetNumberImages(self.wand_ptr) }
	}
	fn image_total_ink_density(&self) -> f64 {
		unsafe { wand_extern::wand::MagickGetImageTotalInkDensity(self.wand_ptr) }
	}
	fn has_next_image(&self) -> bool {
		unsafe { wand_extern::wand::MagickHasNextImage(self.wand_ptr) }
	}
	fn has_previous_image(&self) -> bool {
		unsafe { wand_extern::wand::MagickHasNextImage(self.wand_ptr) }
	}
}

impl Clone for MagickWand {
	fn clone(&self) -> MagickWand {
		let new_wand_ptr;
		unsafe {
			new_wand_ptr = wand_extern::wand::CloneMagickWand(self.wand_ptr)
		}
		MagickWand::new_with_ptr(new_wand_ptr)
	}
}

impl Drop for MagickWand {
	fn finalize(&self) {
		unsafe { wand_extern::wand::DestroyMagickWand(self.wand_ptr); }
	}
}
