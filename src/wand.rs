use types;
use wand_extern;
use pixel_wand::PixelWand;
use pixel;

pub struct MagickWand {
	priv wand_ptr: types::MagickWandPtr
}

pub impl MagickWand {
	fn new() -> MagickWand {
		let ptr;
		unsafe { ptr = wand_extern::NewMagickWand() }
		MagickWand::new_with_ptr(ptr)
	}
	priv fn new_with_ptr(ptr: types::MagickWandPtr) -> MagickWand {
		MagickWand { wand_ptr: ptr }
	}

	fn is_magick_wand(&self) -> bool {
		unsafe { wand_extern::IsMagickWand(self.wand_ptr) }
	}
	fn clear(&self) {
		unsafe { wand_extern::ClearMagickWand(self.wand_ptr) }
	}
	fn adaptive_resize_image(&self, cols: u32, rows: u32) -> bool {
		unsafe { wand_extern::MagickAdaptiveResizeImage(self.wand_ptr, cols, rows) }
	}
	fn read_image(&self, fname: &str) -> bool {
		let path_bytes = str::to_bytes(fname);

		unsafe {
			let raw_path_bytes = vec::raw::to_ptr(path_bytes);
			wand_extern::MagickReadImage(self.wand_ptr, raw_path_bytes as *i8)
		}
	}
	fn write_image(&self, fname: &str) -> bool {
		let path_bytes = str::to_bytes(fname);

		unsafe {
			let raw_path_bytes = vec::raw::to_ptr(path_bytes);
			wand_extern::MagickWriteImage(self.wand_ptr, raw_path_bytes as *i8)
		}
	}
	fn read_imageBlob(&self, blob: &[u8]) -> bool {
		unsafe {
			wand_extern::MagickReadImageBlob(
			  self.wand_ptr,
			  vec::raw::to_ptr(blob),
			  blob.len() as u32)
		}
	}
	fn image_blob(&self) -> ~[u8] {
		unsafe {
			let mut len: u32 = 0;
			let blob = wand_extern::MagickGetImageBlob(self.wand_ptr, &mut len);
			let v: ~[u8] = vec::from_buf(blob as *u8, len as uint);
			wand_extern::MagickRelinquishMemory(blob);
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
			wand_extern::MagickResizeImage(
			  self.wand_ptr,
			  cols,
			  rows,
			  filter,
			  blur)
		}
	}
	fn image_width(&self) -> uint {
		unsafe {
			wand_extern::MagickGetImageWidth(self.wand_ptr) as uint
		}
	}
	fn image_height(&self) -> uint {
		unsafe {
			wand_extern::MagickGetImageHeight(self.wand_ptr) as uint
		}
	}

	/*
	 * Resolve an optional offset/size to sane defaults
	 * which are offset 0, 0, and the max size which can fit
	 * in that area, or the provided values. No bounds checking
	 * is made; the library relies on MagickWand to do all checking
	 */
	priv fn default_offset_and_size(
		  &self,
		  offset: Option<(uint, uint)>,
		  size:   Option<(uint, uint)>) -> ((uint, uint), (uint, uint)) {
		let (x, y) = match offset {
			Some(o) => o,
			None    => (0, 0)
		};

		let (cols, rows) = match size {
			Some(s) => s,
			None    => {
				let (w, h) = (self.image_width(), self.image_height());
				(w-x, h-y)
			}
		};

		return ((x, y), (cols, rows));
	}

	fn export_image_pixels_flat<T: pixel::FromRGB + Copy>(
		  &self,
		  offset: Option<(uint, uint)>,
		  size:   Option<(uint, uint)>) -> Option<~[T]> {

		let ((x, y), (cols, rows)) = self.default_offset_and_size(offset, size);

		let num_pixels = (cols * rows);
		let mut pixel_buffer = vec::with_capacity::<pixel::RGB>(num_pixels);

		unsafe {
			let buffer_ptr = vec::raw::to_ptr(pixel_buffer);
			let success = wand_extern::MagickExportImagePixels(
			  self.wand_ptr,
			  x    as libc::size_t,
			  y    as libc::size_t,
			  cols as libc::size_t,
			  rows as libc::size_t,
			  vec::raw::to_ptr(str::to_bytes("RGB")) as *i8,
			  types::CharPixel,
			  buffer_ptr as *libc::c_void);
			if success {
				vec::raw::set_len::<pixel::RGB>(&mut pixel_buffer, num_pixels);

				//Map to the requested pixel type
				let pixel_buffer: ~[T] = do pixel_buffer.map |p| {
					pixel::FromRGB::from(*p)
				};

				Some(pixel_buffer)
			} else {
				None
			}
		}
	}

	fn export_image_pixels<T : pixel::FromRGB + Copy>(
		  &self,
		  offset: Option<(uint, uint)>,
		  size:   Option<(uint, uint)>) -> Option<~[~[T]]> {

		let (_, (cols, rows)) = self.default_offset_and_size(offset, size);
		let flat_pixels  = match self.export_image_pixels_flat(offset, size) {
			Some(p) => p,
			None    => return None
		};


		//Make it a nested array of pixels
		let mut mapped_pixel_buffer = vec::with_capacity::<~[T]>(rows);
		for uint::range(0, rows) |h| {
			let row = flat_pixels.slice(h * cols, (h+1) * cols).to_owned();
			mapped_pixel_buffer.push(row);
		}
		Some(mapped_pixel_buffer)
	}

	fn import_image_pixels_flat<T : pixel::ToRGB>(
	    &self,
	    pixel_buffer: &[T],
		  offset: Option<(uint, uint)>,
		  size:   Option<(uint, uint)>) -> bool {

		let ((x, y), (cols, rows)) = self.default_offset_and_size(offset, size);

		assert_eq!(cols*rows, pixel_buffer.len());

		let rgb_pixel_buffer = pixel_buffer.map(|p| {
			p.to_rgb()
		});

		unsafe {
			let rgb_buffer_ptr = vec::raw::to_ptr(rgb_pixel_buffer);
			return wand_extern::MagickImportImagePixels(
			  self.wand_ptr,
			  x    as libc::size_t,
			  y    as libc::size_t,
			  cols as libc::size_t,
			  rows as libc::size_t,
			  vec::raw::to_ptr(str::to_bytes("RGB")) as *i8,
			  types::CharPixel,
			  rgb_buffer_ptr as *libc::c_void);
		}
	}

	fn import_image_pixels<T : pixel::ToRGB + Copy>(
	    &self,
	    pixel_buffer: &[~[T]],
		  offset: Option<(uint, uint)>) -> bool {

		let rows = pixel_buffer.len();
		let flat_pixels = vec::concat(pixel_buffer);
		let cols = flat_pixels.len() / pixel_buffer.len();

		return self.import_image_pixels_flat::<T>(flat_pixels, offset, Some((cols, rows)));
	}

	fn new_image(&self, width: u32, height: u32, bg: Option<PixelWand>) -> bool {
		let bg = match bg {
			Some(pw) => pw,
			None     => PixelWand::new()
		};
		unsafe {
			wand_extern::MagickNewImage(
			  self.wand_ptr, width, height, bg.get_ptr())
		}
	}

	fn num_images(&self) -> u32 {
		unsafe { wand_extern::MagickGetNumberImages(self.wand_ptr) }
	}
	fn image_total_ink_density(&self) -> f64 {
		unsafe { wand_extern::MagickGetImageTotalInkDensity(self.wand_ptr) }
	}
	fn has_next_image(&self) -> bool {
		unsafe { wand_extern::MagickHasNextImage(self.wand_ptr) }
	}
	fn has_previous_image(&self) -> bool {
		unsafe { wand_extern::MagickHasNextImage(self.wand_ptr) }
	}
}

impl Clone for MagickWand {
	fn clone(&self) -> MagickWand {
		let new_wand_ptr;
		unsafe {
			new_wand_ptr = wand_extern::CloneMagickWand(self.wand_ptr)
		}
		MagickWand::new_with_ptr(new_wand_ptr)
	}
}

impl Drop for MagickWand {
	fn finalize(&self) {
		unsafe { wand_extern::DestroyMagickWand(self.wand_ptr); }
	}
}
