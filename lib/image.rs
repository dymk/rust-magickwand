//Bug prevents 'use' in this case when compiling a test
mod c_types;

pub struct Image {
	priv img_ptr: c_types::ImagePtr
}

pub impl Image {
	fn new(i: c_types::ImagePtr) -> Image {
		Image { img_ptr: i }
	}
}

impl Drop for Image {
	fn finalize(&self) {
		io::println("TODO: impl destructor for Image");
	}
}
