---@class (exact) nomad.neovim.build.Builder
---@field command string something
---@field fallback function(builder: nomad.neovim.build.Builder) nomad.neovim.build.Builder

---@class (exact) nomad.neovim.build.Builders
---@field cargo function(opts: nomad.neovim.build.CargoOpts?): nomad.neovim.build.Builder
---@field download_prebuilt function(opts: nomad.neovim.build.DownloadPrebuiltOpts?): nomad.neovim.build.Builder
---@field nix function(opts: nomad.neovim.build.NixOpts?): nomad.neovim.build.Builder

---@type nomad.neovim.build.Builders
return {
  cargo = require("nomad.neovim.build.cargo"),
  download_prebuilt = require("nomad.neovim.build.download_prebuilt"),
  nix = require("nomad.neovim.build.nix"),
}
