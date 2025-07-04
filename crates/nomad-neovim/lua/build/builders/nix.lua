---@class (exact) nomad.neovim.build.NixOpts

local future = require("nomad.future")

---@type nomad.neovim.Command
local Command = require("nomad.neovim.command")

---@param exit_code integer
---@return string
local err = function(exit_code)
  return ("Builder 'nix' failed with exit code %s"):format(exit_code)
end

---@param opts nomad.neovim.build.NixOpts
---@param build_ctx nomad.neovim.build.Context
---@return nomad.future.Future<nomad.Result<nil, string>>
return function(opts, build_ctx)
  return future.async(function(ctx)
    local build_res = Command.new("nix")
        :arg("build")
        :arg(".#neovim" .. (vim.version().prerelease and "-nightly" or ""))
        :arg("--accept-flake-config")
        :current_dir(build_ctx:repo_dir())
        :on_stdout(build_ctx.notify)
        :on_stderr(build_ctx.notify)
        :await(ctx)

    if build_res:is_err() then return build_res:map_err(err) end

    return Command.new("find")
        -- Find all the files under /result/lua..
        :args({ "result/lua", "-maxdepth", "1", "-type", "f", })
        -- ..and copy them under /lua, overwriting any existing copies.
        :args({ "-exec", "cp", "-f", "{}", "lua/", ";" })
        :current_dir(build_ctx:repo_dir())
        :on_stderr(build_ctx.notify)
        :await(ctx)
        :map_err(err)
  end)
end
