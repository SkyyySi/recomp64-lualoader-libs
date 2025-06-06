// SPDX-License-Identifier: MIT

mod macros;
mod context;
mod rdram;
mod utils;

#[allow(unused_imports)]
use mlua::prelude::*;

module!(recomp64, lua, {
	rdram   = rdram::recomp64_rdram(lua)?,
	context = context::recomp64_context(lua)?,
	utils   = utils::recomp64_utils(lua)?,
});
