---@class (exact) nomad.neovim.build.Context: nomad.neovim.build.ContextSpec

---@class (exact) nomad.neovim.build.ContextSpec
---
---@field block_on_build fun(build_fut: nomad.future.Future<nomad.Result<nil, string>>, error_lvl: integer)
---@field notify fun(msg: string)

local path = require("nomad.path")

---@generic T
---@param list [T]
---@param start_offset integer
---@param end_offset integer
---@return [T]
local slice = function(list, start_offset, end_offset)
  local sliced = {}
  for idx = start_offset + 1, end_offset do
    sliced[#sliced + 1] = list[idx]
  end
  return sliced
end

local Context = {}
Context.__index = Context

---@param spec nomad.neovim.build.ContextSpec
---@return nomad.neovim.build.Context
Context.new = function(spec)
  ---@cast spec nomad.neovim.build.Context
  return setmetatable(spec, Context)
end

---@param self nomad.neovim.build.Context
---@return nomad.path.Path
function Context:repo_dir()
  if not self._repo_dir then
    local src = debug.getinfo(1, "S").source
    if src:sub(1, 1) ~= "@" then
      error("not a in file source", 2)
    end
    if src:sub(2, 2) ~= path.separator then
      error(("'%s' is not an absolute path"):format(src:sub(2)), 2)
    end
    local file_components = vim.split(src:sub(3), path.separator)
    local repo_components = slice(file_components, 0, #file_components - 5)
    self._repo_dir = path.Path.from_components(repo_components)
  end
  return self._repo_dir
end

return Context
