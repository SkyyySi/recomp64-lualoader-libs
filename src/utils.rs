// SPDX-License-Identifier: MIT

use crate::{module, table};
#[allow(unused_imports)]
use mlua::prelude::*;

pub fn into_module(lua: &Lua, (tb, name): (LuaTable, String)) -> LuaResult<LuaTable> {
	let mt: LuaTable = table!(lua, {
		__name = name,
	})?;

	tb.set_metatable(Some(mt));

	Ok(tb)
}

module!(recomp64_utils, lua, {
	into_module = lua.create_function(into_module)?,
});
