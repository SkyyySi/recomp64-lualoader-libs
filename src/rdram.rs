// SPDX-License-Identifier: MIT

use crate::module;
#[allow(unused_imports)]
use mlua::prelude::*;

#[derive(Clone, Debug, FromLua)]
pub struct RDRAM {
	// TODO
}

impl LuaUserData for RDRAM {
	fn add_fields<F: LuaUserDataFields<Self>>(_fields: &mut F) {
		// TODO
	}

	fn add_methods<M: LuaUserDataMethods<Self>>(_methods: &mut M) {
		// TODO
	}
}

module!(recomp64_rdram, lua, {
	rdram = LuaNil,
});
