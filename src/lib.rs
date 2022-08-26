#![cfg_attr(not(test), no_std)]
#![feature(core_intrinsics)]
#![feature(bench_black_box)]
#![cfg_attr(test, feature(new_uninit))]
#![feature(test)]

#[cfg(test)]
mod bench;
#[cfg_attr(target_arch = "x86_64", path = "x86_64.rs")]
mod imp;

mod private {
	pub trait PixelFormat {
		type E;
	}
}

macro_rules! pixfmt {
	($name:ident $ty:ty) => {
		pub struct $name;

		impl PixelFormat for $name {
			type E = $ty;
		}
	};
}

use core::ptr::NonNull;
use private::PixelFormat;

pixfmt!(Rgbx8888 i32);
pixfmt!(Bgrx8888 i32);

pub struct FrameBuffer<T: PixelFormat> {
	base: NonNull<T::E>,
	width: u16,
	height: u16,
	stride: u16,
}

impl<T: PixelFormat> FrameBuffer<T> {
	/// # Note
	///
	/// width, height and stride are encoded as the real value **minus** one.
	pub unsafe fn new(base: NonNull<T::E>, width: u16, height: u16, stride: u16) -> Self {
		Self {
			base,
			width,
			height,
			stride,
		}
	}
}

unsafe fn read_unaligned_untrusted<T>(ptr: *const T) -> T {
	core::intrinsics::unaligned_volatile_load(ptr)
}
