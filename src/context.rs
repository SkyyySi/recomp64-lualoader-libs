// SPDX-License-Identifier: MIT

use crate::module;
#[allow(unused_imports)]
use mlua::prelude::*;

#[derive(Clone, Debug, FromLua)]
#[allow(unused)]
pub struct Gpr(u64);

#[derive(Clone, Debug, FromLua)]
#[allow(unused)]
pub struct Fpr(f64);

#[derive(Clone, Debug, FromLua)]
#[allow(unused)]
pub struct Context {
	gpr: [Gpr; 32],
	fpr: [Fpr; 32],
	hi: u64,
	lo: u64,
	f_odd: *const u32,
	status_reg: u32,
	mips3_float_mode: u8,
}

impl LuaUserData for Context {
	fn add_fields<F: LuaUserDataFields<Self>>(_fields: &mut F) {
		// TODO
	}

	fn add_methods<M: LuaUserDataMethods<Self>>(_methods: &mut M) {
		// TODO
	}
}

module!(recomp64_context, lua, {
	context = LuaNil,
});

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_context() {
		assert_eq!(size_of::<Context>(), 544);
	}
}
