#!/usr/bin/env lua
--- SPDX-License-Identifier: MIT
---@version 5.4

--------------------------------------------------------------------------------

local _VERSION = _VERSION
local assert   = assert
local error    = error
local pairs    = pairs
local print    = print
local rawequal = rawequal
local require  = require
local tostring = tostring
local type     = type
local debug    = debug
local io       = io
local string   = string
local table    = table

--------------------------------------------------------------------------------

if _VERSION ~= "Lua 5.4" then
	error("This script only supports Lua 5.4! (got: " .. tostring(_VERSION) .. ")")
end

--------------------------------------------------------------------------------

local debug_getmetatable = debug.getmetatable

local io_flush = io.flush
local io_write = io.write

local string_format = string.format
local string_match  = string.match
local string_rep    = string.rep

local table_pack   = table.pack
local table_sort   = table.sort
local table_unpack = table.unpack

--------------------------------------------------------------------------------

---@param value any
---@param depth integer
---@return string
local function repr(value, depth)
	local ty = type(value)

	if ty == "string" then
		---@cast value string
		return string_format("%q", value)
	elseif ty == "table" then
		---@cast value table

		local value_is_empty = next(value, nil) == nil
		local mt_is_empty = true

		local result = ""

		local mt = debug_getmetatable(value)
		if type(mt) == "table" then
			mt_is_empty = next(mt, nil) == nil

			---@type (fun(self: table): string)?
			local mt_tostring = rawget(mt, "__tostring")

			if mt_tostring then
				return mt_tostring(value)
			elseif value_is_empty and mt_is_empty then
				return result .. "{}"
			end
		end

		result = result .. "{"
		if not (value_is_empty and mt_is_empty) then
			result = result .. "\n"
		end
		depth  = depth + 1

		---@type [any, string, string][]
		local items = {}
		for k, v in pairs(value) do
			local key_repr
			if type(k) == "string" and string_match(k, "^[a-zA-Z_][a-zA-Z0-9_]*$") then
				key_repr = k
			else
				key_repr = "[" .. repr(k, depth) .. "]"
			end

			items[#items+1] = { k, key_repr, repr(v, depth) }
		end
		table_sort(items, function(a, b)
			return a[2] < b[2]
		end)

		local indent = string_rep("    ", depth)
		for i = 1, #items do
			local item = items[i]

			local key        = item[1]
			local key_repr   = item[2]
			local value_repr = item[3]

			result = result .. indent .. key_repr .. " = " .. value_repr

			if (i < #items) then
				result = result .. ",\n"
			end
		end

		if not rawequal(mt, nil) then
			if not value_is_empty then
				result = result .. ",\n"
			end

			result = result .. indent .. "\027[1;3;35m<>\027[0m = " .. repr(mt, depth)
		end

		depth  = depth - 1
		if not (value_is_empty and mt_is_empty) then
			result = result .. "\n" .. string_rep("    ", depth)
		end
		result = result .. "}"

		return result
	end

	return tostring(value)
end

---@param ... any
local function display(...)
	---@type { [integer]: any, n: integer }
	local args = table_pack(...)

	for i = 1, args.n do
		io_write(repr(args[i], 0))

		if i < args.n then
			io_write(", ")
		end
	end

	io_write("\n")
	io_flush()
end

--------------------------------------------------------------------------------

local recomp64 = require("recomp64")
--display(recomp64)

print("Loading RDRAM dump from disk...")
local rdram = recomp64.rdram.new_from_file("./rdram-dump.bin")
print("Done!")
display(rdram)
