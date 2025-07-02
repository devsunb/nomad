---@class (exact) nomad.neovim.build
---
---Builders.
---@field builders nomad.neovim.build.builders
---
---Drivers.
---@field drivers nomad.neovim.build.drivers

---@type nomad.neovim.build
return {
  builders = require("nomad.neovim.build.builders"),
  drivers = require("nomad.neovim.build.drivers"),
}
