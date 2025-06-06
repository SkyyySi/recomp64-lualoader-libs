#!/usr/bin/env lua
--- SPDX-License-Identifier: MIT
---@version 5.4
assert(_VERSION == "Lua 5.4")

--------------------------------------------------------------------------------

local recomp64 = require("recomp64")

print(string.format("recomp64 = %s", tostring(recomp64)))
for k, v in pairs(recomp64) do
	print(string.format("recomp64.%s = %s", tostring(k), tostring(v)))
end
