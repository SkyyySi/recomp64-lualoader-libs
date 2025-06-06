// SPDX-License-Identifier: MIT

use crate::module;
use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
use std::ffi::c_void;
use std::mem::align_of;
use std::path::PathBuf;
use std::ptr::null_mut;
#[allow(unused_imports)]
use mlua::prelude::*;

#[derive(Debug, FromLua)]
#[allow(unused)]
pub struct RDRAM {
	data: *mut u8,
	owns_data: bool,
	capacity: usize,
}

impl RDRAM {
	pub fn new(data: *mut u8) -> Self {
		Self {
			data,
			owns_data: false,
			capacity: 0x20000000,
		}
	}

	#[allow(unused)]
	pub fn new_from_file(path: PathBuf) -> LuaResult<Self> {
		let data: Vec<u8> = std::fs::read(path)?;
		Ok(Self::from(data))
	}
}

impl From<Vec<u8>> for RDRAM {
	fn from(vec: Vec<u8>) -> Self {
		let layout: Layout = Layout::from_size_align(
			vec.len(),
			align_of::<u8>(),
		).unwrap();

		let data: *mut u8 = unsafe {
			alloc(layout)
		};

		if data.is_null() {
			handle_alloc_error(layout);
		}

		vec.iter().enumerate().for_each(|(i, byte_ref)| {
			let byte: u8 = *byte_ref;
			let ptr: *mut u8 = data.wrapping_add(i);
			unsafe {
				*ptr = byte;
			};
		});

		Self {
			data,
			owns_data: true,
			capacity: vec.len(),
		}
	}
}

impl FromIterator<u8> for RDRAM {
	fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
		let vec: Vec<u8> = iter.into_iter().collect();
		Self::from(vec)
	}
}

impl Drop for RDRAM {
	fn drop(&mut self) {
		if !self.owns_data {
			return;
		}

		let layout: Layout = Layout::from_size_align(
			self.capacity,
			align_of::<u8>(),
		).unwrap();

		unsafe {
			dealloc(self.data, layout);
		}

		self.data = null_mut();
	}
}

impl Clone for RDRAM {
	fn clone(&self) -> Self {
		let mut buffer: Vec<u8> = Vec::with_capacity(self.capacity);

		for i in 0..self.capacity {
			let value_ptr: *mut u8 = self.data.wrapping_add(i);
			let value: u8 = unsafe {
				*value_ptr
			};
			buffer.push(value);
		}

		Self::from(buffer)
	}
}

impl LuaUserData for RDRAM {
	fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
		fields.add_field_method_get("data", |_, this| {
			Ok(LuaLightUserData(this.data as *mut c_void))
		});
	}

	fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
		methods.add_meta_method("__tostring", |_, this, _: ()| {
			Ok(format!("{this:?}"))
		});
	}
}

module!(recomp64_rdram, lua, {
	rdram = LuaNil,
	new = lua.create_function(|_lua, data: LuaLightUserData| {
		Ok(RDRAM::new(data.0 as *mut u8))
	})?,
	new_from_file = lua.create_function(|_lua, data: String| {
		Ok(RDRAM::new_from_file(data.into()))
	})?,
});
