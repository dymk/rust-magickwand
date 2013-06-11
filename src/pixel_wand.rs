use types;
use wand_extern;

pub struct PixelWand {
	pxwand_ptr: types::PixelWandPtr
}

pub impl PixelWand {
	fn new() -> PixelWand {
		let ptr;
		unsafe {
			ptr = wand_extern::NewPixelWand();
		}
		PixelWand { pxwand_ptr: ptr }
	}

	fn is_pixel_wand(&self) -> bool {
		unsafe {
			wand_extern::IsPixelWand(self.pxwand_ptr)
		}
	}

	unsafe fn get_ptr(&self) -> types::PixelWandPtr {
		return self.pxwand_ptr;
	}
}

impl Drop for PixelWand {
	fn finalize(&self) {
		unsafe {
			wand_extern::DestroyPixelWand(self.pxwand_ptr);
		}
	}
}
