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
	raw_data: *mut u8,
	owns_raw_data: bool,
	capacity: usize,
}

impl RDRAM {
	pub fn new(data: *mut u8) -> Self {
		Self {
			raw_data: data,
			owns_raw_data: false,
			capacity: 0x20000000,
		}
	}

	pub fn new_from_file(path: PathBuf) -> LuaResult<Self> {
		let data: Vec<u8> = std::fs::read(path)?;
		Ok(Self::from(data))
	}

	pub fn index(&self, index: LuaInteger) -> LuaResult<u8> {
		if (index < 0) || (index as usize > self.capacity) {
			return Err(LuaError::RuntimeError(format!(
				"Index out of range! (expected a value in range [1, {}], got {})",
				self.capacity,
				index,
			)));
		}

		let index_usize: usize = (index - 1)
			.try_into()
			.map_err(|err: std::num::TryFromIntError| LuaError::FromLuaConversionError {
				from: "number",
				to: "usize".to_string(),
				message: Some(err.to_string())
			})?;

		let real_index: usize = index_usize ^ 3;

		if real_index >= self.capacity {
			return Err(LuaError::RuntimeError(
				"Index out of range!".to_string()
			));
		}

		let ptr: *mut u8 = self.raw_data.wrapping_add(real_index);
		let byte: u8 = unsafe { *ptr };
		Ok(byte)
	}

	/* fn index_with_number(&self, key: LuaInteger) -> LuaResult<LuaValue> {
		if (key < 1) || (key as usize > self.capacity) {
			return Err(LuaError::RuntimeError(format!(
				"Index out of range! (expected a value in range [1, {}], got {})",
				self.capacity,
				key,
			)))
		}

		let key_usize: usize = (key - 1)
			.try_into()
			.map_err(|err: std::num::TryFromIntError| LuaError::FromLuaConversionError {
				from: "number",
				to: "usize".to_string(),
				message: Some(err.to_string())
			})?;

		let ptr: *mut u8 = self.raw_data.wrapping_add(key_usize);
		let byte: u8 = unsafe { *ptr };

		Ok(LuaValue::Integer(byte as LuaInteger))
	}

	pub fn index(&self, wrapped_key: LuaValue) -> LuaResult<LuaValue> {
		match wrapped_key {
			LuaValue::Integer(key) => self.index_with_number(key),
			LuaValue::Number(key) => self.index_with_number(key as LuaInteger),
			_ => return Err(LuaError::BadArgument {
				to: Some("RDRAM.__index".to_string()),
				pos: 2,
				name: Some("key".to_string()),
				cause: std::sync::Arc::new(LuaError::RuntimeError(format!(
					"string or integer expected, got {}",
					wrapped_key.type_name(),
				))),
			}),
		}
	} */

	pub fn len(&self) -> usize {
		let mut length: usize = self.capacity;

		while length > 0 {
			let ptr: *mut u8 = self.raw_data.wrapping_add(length - 1);
			let byte: u8 = unsafe { *ptr };
			if byte != 0 {
				break;
			}
			length -= 1;
		}

		for _ in (!(length & 0b11))..=0b11 {
			let index: usize = length ^ 0b11;
			if index >= self.capacity {
				continue;
			}
			let ptr: *mut u8 = self.raw_data.wrapping_add(index);
			let byte: u8 = unsafe { *ptr };
			if byte != 0 {
				length += 1;
			}
		}

		length
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
			raw_data: data,
			owns_raw_data: true,
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
		if !self.owns_raw_data {
			return;
		}

		let layout: Layout = Layout::from_size_align(
			self.capacity,
			align_of::<u8>(),
		).unwrap();

		unsafe {
			dealloc(self.raw_data, layout);
		}

		self.raw_data = null_mut();
	}
}

impl Clone for RDRAM {
	fn clone(&self) -> Self {
		let mut buffer: Vec<u8> = Vec::with_capacity(self.capacity);

		for i in 0..self.capacity {
			let value_ptr: *mut u8 = self.raw_data.wrapping_add(i);
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
		fields.add_field_method_get("raw_data", |_, this| {
			Ok(LuaLightUserData(this.raw_data as *mut c_void))
		});
		fields.add_field_method_get("owns_raw_data", |_, this| {
			Ok(this.owns_raw_data)
		});
		fields.add_field_method_get("capacity", |_, this| {
			Ok(this.capacity)
		});
	}

	fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
		let len_method = |_: &Lua, this: &RDRAM, _: ()| -> LuaResult<LuaInteger> {
			Ok(this.len() as LuaInteger)
		};
		methods.add_method("len", len_method);
		methods.add_meta_method("__len", len_method);

		methods.add_meta_method("__tostring", |_, this, _: ()| {
			Ok(format!("{this:?}"))
		});

		methods.add_meta_method("__index", |_, this, index: LuaInteger| {
			this.index(index)
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
