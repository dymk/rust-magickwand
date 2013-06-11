#[ crate_type = "lib" ];
#[ link(name = "magick_wand",
	      vers = "0.0.1",
	      uuid = "bedfd060-afb0-11e2-9e96-0800200c9a66",
	      url  = "https://github.com/dymk/rust-magickwand")];

pub mod wand;
pub mod types;
pub mod pixel;
pub mod pixel_wand;
pub mod image;
pub mod wand_extern;

#[path="../test/test_all.rs"]
#[cfg(test)]
mod test_all;
