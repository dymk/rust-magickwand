pub trait FromRGBData {
	fn fromRGBData(c: RGB) -> Self;
}

pub struct RGB(u8, u8, u8);
pub impl RGB {
	#[inline(always)]
	fn red(&self) -> u8 {
		let RGB(r, _, _) = *self;
		r
	}
	#[inline(always)]
	fn green(&self) -> u8 {
		let RGB(_, g, _) = *self;
		g
	}
	#[inline(always)]
	fn blue(&self) -> u8 {
		let RGB(_, _, b) = *self;
		b
	}
}

pub struct YIQ(u8, i8, i8);
pub impl YIQ {
	#[inline(always)]
	fn luma(&self) -> u8 {
		let YIQ(y, _, _) = *self;
		y
	}
	#[inline(always)]
	fn inphase(&self) -> i8 {
		let YIQ(_, i, _) = *self;
		i
	}
	#[inline(always)]
	fn quad(&self) -> i8 {
		let YIQ(_, _, q) = *self;
		q
	}
}

impl Eq for RGB {
	#[inline(always)]
	fn eq(&self, &other: &RGB) -> bool {
		let RGB(r0, g0, b0) = *self;
		let RGB(r1, g1, b1) = other;
		r0 == r1 && g0 == g1 && b0 == b1
	}
	#[inline(always)]
	fn ne(&self, &other: &RGB) -> bool {
		return !(self == &other);
	}
}

impl Eq for YIQ {
	#[inline(always)]
	fn eq(&self, &other: &YIQ) -> bool {
		let YIQ(y0, i0, q0) = *self;
		let YIQ(y1, i1, q1) = other;
		y0 == y1 && i0 == i1 && q0 == q1
	}
	#[inline(always)]
	fn ne(&self, &other: &YIQ) -> bool {
		return !(self == &other);
	}
}

impl FromRGBData for RGB {
	#[inline(always)]
	fn fromRGBData(c: RGB) -> RGB {
		c
	}
}

impl FromRGBData for YIQ {
	fn fromRGBData(c: RGB) -> YIQ {
		/*
		 * Thank you kindly,
		 * http://www.eembc.org/techlit/datasheets/yiq_consumer.pdf
		 */
		let RGB(r, g, b) = c;
		let r = r as float;
		let g = g as float;
		let b = b as float;

		let y = (
		  (0.299 * r) +
		  (0.587 * g) +
		  (0.114 * b)  ) as u8;
		let i = (
		  (0.596 * r) -
		  (0.275 * g) -
		  (0.321 * b)  ) as i8;
		let q = (
		  (0.212 * r) -
		  (0.523 * g) +
		  (0.311 * b)  ) as i8;
		YIQ(y, i, q)
	}
}
