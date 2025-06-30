---@class (exact) nomad.neovim.Build
---@field build function(builder: nomad.neovim.build.Builder)
---@field builders nomad.neovim.build.Builders

---@type nomad.neovim.Build
return {
  build = build,
  builders = require("nomad.neovim.build.builders"),
}
