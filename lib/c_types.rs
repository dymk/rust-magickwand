pub type MagickWandPtr = *libc::c_void;
pub type ImagePtr = *libc::c_void;

// "magick/constitute.h", line 25
pub enum StorageType {
  UndefinedPixel,
  CharPixel,
  DoublePixel,
  FloatPixel,
  IntegerPixel,
  LongPixel,
  QuantumPixel,
  ShortPixel
}
