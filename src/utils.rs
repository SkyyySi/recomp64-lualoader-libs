// SPDX-License-Identifier: MIT

use crate::{module, table};
#[allow(unused_imports)]
use mlua::prelude::*;

pub fn name_of(_lua: &Lua, value: LuaValue) -> LuaResult<Option<String>> {
	use mlua::Value as Lv;

	match value {
		Lv::Table(inner) => Ok(inner
			.metatable()
			.and_then(|mt| mt.raw_get("__name").ok())
		),
		Lv::UserData(inner) => inner.metatable()?.get("__name"),
		Lv::LightUserData(_inner) => todo!(),
		_ => Ok(None),
	}
}

pub fn into_module(lua: &Lua, (tb, name): (LuaTable, String)) -> LuaResult<LuaTable> {
	let mt: LuaTable = table!(lua, {
		__name = name,
		__index = lua.create_function(|lua, (this, key): (LuaTable, LuaValue)| -> LuaResult<()> {
			let name: String = name_of(lua, LuaValue::Table(this))?
				.unwrap_or("<unknown>".to_string());

			Err(LuaError::RuntimeError(format!(
				"Unknown member {key:?} for module {name:?}!",
			)))
		})?,
		__newindex = lua.create_function(|lua, (this, key, value): (LuaTable, LuaValue, LuaValue)| -> LuaResult<()> {
			let name: String = name_of(lua, LuaValue::Table(this))?
				.unwrap_or("<unknown>".to_string());

			Err(LuaError::RuntimeError(format!(
				"Inserting new values into module {name:?} is not allowed! (key={key:?}, value={value:?})",
			)))
		})?,
	})?;

	tb.set_metatable(Some(mt));

	Ok(tb)
}

module!(recomp64_utils, lua, {
	name_of = lua.create_function(name_of)?,
	into_module = lua.create_function(into_module)?,
});
