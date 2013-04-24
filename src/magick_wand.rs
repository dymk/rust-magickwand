#[ crate_type = "lib" ];
#[ link(name = "magick_wand",
	      vers = "0.0.1")];

pub mod wand;

/*
 * This also can't be exported yet due to issue 5951
 */
// pub mod wand_extern;

/*
 * For testing, see test_magick_wand.rs,
 * which is a file used to workaround issue
 * https://github.com/mozilla/rust/issues/5951
 */
/*
 * mod test_magick_wand;
 */
