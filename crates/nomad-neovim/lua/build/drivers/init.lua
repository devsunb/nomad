---@class (exact) nomad.neovim.build.drivers
---
---Drive the builder to completion within lazy.nvim's `build` function.
---@field lazy nomad.neovim.build.Driver

---@type nomad.neovim.build.drivers
return {
  lazy = require("nomad.neovim.build.drivers.lazy"),
}
