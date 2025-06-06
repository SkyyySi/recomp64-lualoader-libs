#!/usr/bin/env bash
set -uCo pipefail
clear
cd "$(dirname -- "${BASH_SOURCE[0]}")" || exit 1

eval "$(luarocks path --lua-version '5.4')"
export LUA_CPATH="$PWD/target/debug/lib?.so;$LUA_CPATH"

function print_horizontal_separator() {
	local -i columns="${COLUMNS:-$(tput cols)}"

	if (( columns < 1 )); then
		columns=120
	fi

	printf '%*s' $(( columns - 1 )) '' | sed 's| |─|g'
	printf '\n'
}

if cargo build; then
	print_horizontal_separator
	lua5.4 -- './test.lua'
fi
