// SPDX-License-Identifier: MIT

#[macro_export(local_inner_macros)]
macro_rules! table {
	($lua:ident, $body:tt $( , $mt_body:tt )? $(,)?) => {
		($crate::table!(@parse_body, $lua, $body))
		$(
			.inspect(|table| table.set_metatable(
				($crate::table!(@parse_body, $lua, $mt_body)).ok()
			))
		)?
	};

	(@parse_body, $lua:ident, { $( $key:tt = $value:expr ),* $(,)? }) => {
		::mlua::Lua::create_table($lua)
		$(
			.and_then(|table| table.raw_set(
				($crate::table!(@parse_key, $key)),
				($value),
			).map(|_| table))
		)*
	};

	(@parse_key, [ $key:expr ]) => {
		$key
	};

	(@parse_key, $key:ident) => {
		::std::stringify!($key)
	};
}

#[macro_export(local_inner_macros)]
macro_rules! module {
	($name:ident, $lua:ident, $body:tt $(,)?) => {
		#[::mlua::lua_module]
		pub fn $name($lua: &::mlua::Lua) -> ::mlua::Result<::mlua::Table> {
			$crate::table!($lua, $body)
				.and_then(|tb| $crate::utils::into_module($lua, (tb, (::std::stringify!($name).to_string()))))
		}
	};
}
