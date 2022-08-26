use super::*;
use core::arch::x86_64;

impl FrameBuffer<Rgbx8888> {
	/// # Note
	///
	/// width, height and stride are encoded as the real value **minus** one.
	#[target_feature(enable = "ssse3")]
	pub unsafe fn copy_from_raw_untrusted_rgb24_to_rgbx32(
		&mut self,
		src: *const [u8; 3],
		stride: u16,
		x: u16,
		y: u16,
		w: u16,
		h: u16,
	) {
		let shuf = x86_64::_mm_set_epi8(-1, 11, 10, 9, -1, 8, 7, 6, -1, 5, 4, 3, -1, 2, 1, 0);
		copy_from_raw_untrusted_rgb24_to_any32(self, src, stride, x, y, w, h, shuf, |a, b, c| {
			[a, b, c, 0]
		})
	}
}

impl FrameBuffer<Bgrx8888> {
	/// # Note
	///
	/// width, height and stride are encoded as the real value **minus** one.
	#[target_feature(enable = "ssse3")]
	pub unsafe fn copy_from_raw_untrusted_rgb24_to_bgrx32(
		&mut self,
		src: *const [u8; 3],
		stride: u16,
		x: u16,
		y: u16,
		w: u16,
		h: u16,
	) {
		let shuf = x86_64::_mm_set_epi8(-1, 9, 10, 11, -1, 6, 7, 8, -1, 3, 4, 5, -1, 0, 1, 2);
		copy_from_raw_untrusted_rgb24_to_any32(self, src, stride, x, y, w, h, shuf, |a, b, c| {
			[c, b, a, 0]
		})
	}
}

/// # Note
///
/// width, height and stride are encoded as the real value **minus** one.
#[target_feature(enable = "ssse3")]
unsafe fn copy_from_raw_untrusted_rgb24_to_any32(
	slf: &mut FrameBuffer<impl PixelFormat>,
	src: *const [u8; 3],
	stride: u16,
	x: u16,
	y: u16,
	w: u16,
	h: u16,
	shuf: x86_64::__m128i,
	shuf_32: impl Fn(u8, u8, u8) -> [u8; 4] + Copy,
) {
	let f = |n| usize::from(n) + 1;
	let (stride, x, y, w, h) = (f(stride), usize::from(x), usize::from(y), f(w), f(h));
	assert!(x < f(slf.width) && x + w <= f(slf.width));
	assert!(y < f(slf.height) && y + h <= f(slf.height));
	let mut src = src.cast::<u8>();
	let mut dst = slf.base.as_ptr().add(x).cast::<u8>().add(y * f(slf.stride));
	let pre_end = dst.add((h - 1) * f(slf.stride));
	while dst != pre_end {
		copy_untrusted_row_rgb24_to_any32(dst.cast(), src.cast(), w, false, shuf, shuf_32);
		src = src.add(stride);
		dst = dst.add(f(slf.stride));
	}
	copy_untrusted_row_rgb24_to_any32(dst.cast(), src.cast(), w, true, shuf, shuf_32);
}

#[target_feature(enable = "ssse3")]
unsafe fn copy_untrusted_row_rgb24_to_any32(
	mut dst: *mut i32,
	mut src: *const [u8; 3],
	w: usize,
	last: bool,
	shuf: x86_64::__m128i,
	shuf_32: impl Fn(u8, u8, u8) -> [u8; 4] + Copy,
) {
	#[repr(C)]
	struct E(u16, u8);
	let end = dst.add(w);
	// Special-case w <= 4 so the much more common loop is simpler & faster
	if w <= 4 {
		while dst != end {
			let E(a, c) = read_unaligned_untrusted(src.cast::<E>());
			let [a, b] = a.to_le_bytes();
			*dst = i32::from_le_bytes(shuf_32(a, b, c));
			src = src.add(1);
			dst = dst.add(1);
		}
	} else {
		// Align 16
		while dst as usize & 0b1111 != 0 {
			let [a, b, c, _] = read_unaligned_untrusted(src.cast::<i32>()).to_le_bytes();
			*dst = i32::from_le_bytes(shuf_32(a, b, c));
			src = src.add(1);
			dst = dst.add(1);
		}
		// Loop 16
		let mut end_16 = (end as usize & !0b1111) as *mut i32;
		// Be careful with out of bounds reads
		if last && end_16 == end {
			end_16 = end_16.sub(4);
		}
		while dst != end_16 {
			let v = read_unaligned_untrusted(src.cast::<x86_64::__m128i>());
			let v = x86_64::_mm_shuffle_epi8(v, shuf);
			dst.cast::<x86_64::__m128i>().write(v);
			src = src.add(4);
			dst = dst.add(4);
		}
		// Copy remaining bytes (up to 16)
		// Be careful again
		while dst != end {
			let E(a, c) = read_unaligned_untrusted(src.cast::<E>());
			let [a, b] = a.to_le_bytes();
			dst.write(i32::from_le_bytes(shuf_32(a, b, c)));
			src = src.add(1);
			dst = dst.add(1);
		}
	}
}
