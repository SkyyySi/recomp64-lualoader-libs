--- SPDX-License-Identifier: MIT
---@meta recomp64.rdram

---@class recomp64.rdram.RDRAM : userdata

---@class recomp64.rdram : table
---@field new           fun(data: lightuserdata): recomp64.rdram.RDRAM
---@field new_from_file fun(path: string): recomp64.rdram.RDRAM
local _M = {}

-- ...

return _M
