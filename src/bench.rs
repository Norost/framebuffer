extern crate test;

use super::*;
use std::hint::black_box;
use test::Bencher;

fn with_fb<T>(stride: u16, width: u16, height: u16, f: impl FnOnce(&mut FrameBuffer<T>))
where
	T: PixelFormat,
{
	let mut buf = Box::<[T::E]>::new_zeroed_slice(usize::from(stride) * usize::from(height));
	let mut fb = FrameBuffer {
		base: NonNull::from(&mut *buf).cast(),
		stride,
		width,
		height,
	};
	f(&mut fb)
}

#[bench]
fn copy_untrusted_rgb24_to_bgrx32(b: &mut Bencher) {
	static SRC: [[u8; 3]; 640 * 480] = [[0; 3]; 640 * 480];
	with_fb(1024, 640, 480, |fb| {
		let (src, stride, x, y, w, h) = black_box((&SRC[..], 640, 0, 0, 640, 480));
		b.bytes = u64::from(w) * u64::from(h) * 3;
		b.iter(|| unsafe {
			fb.copy_from_raw_untrusted_rgb24_to_bgrx32(src.as_ptr(), stride, x, y, w, h)
		})
	});
}

#[bench]
fn copy_untrusted_rgb24_to_bgrx32_margin(b: &mut Bencher) {
	static SRC: [[u8; 3]; 640 * 480] = [[0; 3]; 640 * 480];
	with_fb(1024, 640, 480, |fb| {
		let (src, stride, x, y, w, h) = black_box((&SRC[..], 640, 2, 2, 636, 476));
		b.bytes = u64::from(w) * u64::from(h) * 3;
		b.iter(|| unsafe {
			fb.copy_from_raw_untrusted_rgb24_to_bgrx32(src.as_ptr(), stride, x, y, w, h)
		})
	});
}

#[bench]
fn copy_untrusted_rgb24_to_bgrx32_large(b: &mut Bencher) {
	static SRC: [[u8; 3]; 1920 * 1080] = [[0; 3]; 1920 * 1080];
	with_fb(2048, 1920, 1080, |fb| {
		let (src, stride, x, y, w, h) = black_box((&SRC[..], 1920, 0, 0, 1920, 1080));
		b.bytes = u64::from(w) * u64::from(h) * 3;
		b.iter(|| unsafe {
			fb.copy_from_raw_untrusted_rgb24_to_bgrx32(src.as_ptr(), stride, x, y, w, h)
		})
	});
}

#[bench]
fn copy_untrusted_rgb24_to_bgrx32_blit_small(b: &mut Bencher) {
	static SRC: [[u8; 3]; 1920 * 1080] = [[0; 3]; 1920 * 1080];
	with_fb(2048, 1920, 1080, |fb| {
		let (src, stride, x, y, w, h) = black_box((&SRC[..], 1920, 60, 79, 33, 35));
		b.bytes = u64::from(w) * u64::from(h) * 3;
		b.iter(|| unsafe {
			fb.copy_from_raw_untrusted_rgb24_to_bgrx32(src.as_ptr(), stride, x, y, w, h)
		})
	});
}
