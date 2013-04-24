/*
 * This file is a placeholder while
 * https://github.com/mozilla/rust/issues/5951
 * is worked out.
 * For now, all tests are put in here,
 * and this file is built and run
 * to test the magick_wand crate
 */

/*
 * Only one of the following at a time can be run
 */
#[path = "../test/test_wand.rs"]
mod test_wand;
// #[path = "../test/test_wand_extern.rs"]
// mod test_wand_extern;

#[path = "../test/test_image.rs"]
mod test_image;
