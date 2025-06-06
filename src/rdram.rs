// SPDX-License-Identifier: MIT

use crate::table;
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

#[mlua::lua_module]
pub fn recomp64_rdram(lua: &Lua) -> LuaResult<LuaTable> {
	table!(lua, {
		// TODO
		rdram = LuaNil,
	})
}
