// SPDX-License-Identifier: MIT

mod macros;
mod context;
mod rdram;
mod utils;

use mlua::prelude::*;

#[mlua::lua_module]
pub fn recomp64(lua: &Lua) -> LuaResult<LuaTable> {
	table!(lua, {
		rdram = rdram::recomp64_rdram(lua)?,
		context = context::recomp64_context(lua)?,
		utils = utils::recomp64_utils(lua)?,
	})
}
