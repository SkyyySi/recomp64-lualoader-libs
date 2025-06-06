// SPDX-License-Identifier: MIT

use crate::table;
use mlua::prelude::*;

#[mlua::lua_module]
pub fn recomp64_utils(lua: &Lua) -> LuaResult<LuaTable> {
	table!(lua, {
		// TODO
	})
}
