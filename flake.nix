{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    flake-parts.url = "github:hercules-ci/flake-parts";

    flake-root.url = "github:srid/flake-root";

    crane.url = "github:ipetkov/crane";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    neovim-nightly-overlay = {
      url = "github:nix-community/neovim-nightly-overlay/master";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-parts.follows = "flake-parts";
      inputs.treefmt-nix.follows = "treefmt-nix";
    };

    nix-develop-gha = {
      url = "github:nicknovitski/nix-develop";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];

      imports = [
        inputs.flake-root.flakeModule
        inputs.treefmt-nix.flakeModule
        ./nix/crane.nix
        ./nix/formatter.nix
      ];

      perSystem =
        {
          config,
          pkgs,
          lib,
          inputs',
          crane,
          ...
        }:
        let
          common = {
            devShell = crane.lib.devShell {
              inherit (config) checks;
            };
          };

          neovim =
            let
              neovimPkg =
                isNightly: if isNightly then inputs'.neovim-nightly-overlay.packages.default else pkgs.neovim;

              mkOverride =
                {
                  isNightly,
                }:
                (drv: {
                  nativeBuildInputs = (drv.nativeBuildInputs or [ ]) ++ [
                    (neovimPkg isNightly)
                  ];
                });

              mkDevShell =
                {
                  isNightly,
                }:
                common.devShell.overrideAttrs (mkOverride {
                  inherit isNightly;
                });

              mkPlugin =
                {
                  isNightly,
                  isRelease ? true,
                }:
                crane.lib.buildPackage (
                  crane.commonArgs
                  // (
                    let
                      # Get the crate's name and version.
                      crateInfos = builtins.fromJSON (
                        builtins.readFile (
                          pkgs.runCommand "cargo-metadata"
                            {
                              nativeBuildInputs = [
                                crane.lib.cargo
                                pkgs.jq
                              ];
                            }
                            ''
                              cd ${crane.commonArgs.src}
                              cargo metadata \
                                --format-version 1 \
                                --no-deps \
                                --offline \
                                --manifest-path crates/mad-neovim/Cargo.toml | \
                              jq '
                                .workspace_default_members[0] as $default_id |
                                .packages[] |
                                select(.id == $default_id) |
                                {pname: .name, version: .version}
                              ' > $out
                            ''
                        )
                      );
                    in
                    {
                      inherit (crateInfos) pname version;
                      doCheck = false;
                      # We'll handle the installation ourselves.
                      doNotPostBuildInstallCargoBinaries = true;
                      buildPhaseCargoCommand =
                        let
                          nightlyFlag = lib.optionalString isNightly "--nightly";
                          releaseFlag = lib.optionalString isRelease "--release";
                        in
                        "cargo xtask build ${nightlyFlag} ${releaseFlag}";
                      installPhaseCommand = ''
                        mkdir -p $out
                        mv lua/* $out/
                      '';
                    }
                  )
                );
              mkTests =
                {
                  isNightly,
                }:
                crane.lib.cargoTest (
                  crane.commonArgs
                  // {
                    cargoTestExtraArgs = lib.concatStringsSep " " [
                      "--package=tests"
                      "--features=neovim${lib.optionalString isNightly "-nightly"}"
                      "--no-fail-fast"
                    ];
                    nativeBuildInputs = (crane.commonArgs.nativeBuildInputs or [ ]) ++ [
                      (neovimPkg isNightly)
                    ];
                  }
                );
            in
            {
              checks = {
                test = mkTests { isNightly = false; };
                test-nightly = mkTests { isNightly = true; };
              };
              devShells = {
                zero-dot-eleven = mkDevShell { isNightly = false; };
                nightly = mkDevShell { isNightly = true; };
              };
              packages = {
                zero-dot-eleven = mkPlugin { isNightly = false; };
                nightly = mkPlugin { isNightly = true; };
              };
            };
        in
        {
          apps =
            {
              nix-develop-gha = {
                type = "app";
                program = "${inputs'.nix-develop-gha.packages.default}/bin/nix-develop-gha";
              };
            }
            # Workaround for https://github.com/NixOS/nix/issues/8881 so that
            # we can run individual checks with `nix run .#check-<foo>`.
            // lib.mapAttrs' (name: check: {
              name = "check-${name}";
              value = {
                type = "app";
                program = "${check}";
              };
            }) config.checks;
          checks = {
            clippy = crane.lib.cargoClippy (
              crane.commonArgs
              // {
                cargoClippyExtraArgs = lib.concatStringsSep " " [
                  "--all-features"
                  "--all-targets"
                  "--no-deps"
                  "--workspace"
                  "--"
                  "--deny warnings"
                ];
              }
            );
            docs = crane.lib.cargoDoc (
              crane.commonArgs
              // {
                cargoDocExtraArgs = lib.concatStringsSep " " [
                  "--all-features"
                  "--no-deps"
                  "--workspace"
                ];
                env = {
                  RUSTFLAGS = "--deny warnings";
                };
              }
            );
            fmt = config.treefmt.build.check inputs.self;
            test-neovim = neovim.checks.test;
            test-neovim-nightly = neovim.checks.test-nightly;
          };
          packages = {
            coverage = crane.lib.cargoLlvmCov (
              crane.commonArgs
              // {
                buildPhaseCargoCommand = ''
                  # Run unit tests.
                  (cd crates && cargo llvm-cov test --no-report)

                  # Run integration tests.
                  (cd tests && cargo llvm-cov test --no-report --features=auth,collab,mock,walkdir)

                  # Generate coverage report.
                  cargo llvm-cov report --codecov --output-path codecov.json
                '';
                installPhaseCommand = ''
                  mkdir -p $out
                  mv codecov.json $out/
                '';
                env = (crane.commonArgs.env or { }) // {
                  # Setting this will disable some tests that fail in headless
                  # environments like CI.
                  HEADLESS = "true";
                };
              }
            );
            neovim = neovim.packages.zero-dot-eleven;
            neovim-nightly = neovim.packages.nightly;
          };
          devShells = {
            default = common.devShell;
            neovim = neovim.devShells.zero-dot-eleven;
            neovim-nightly = neovim.devShells.nightly;
          };
        };
    };

  nixConfig = {
    extra-substituters = [ "https://nix-community.cachix.org" ];
    extra-trusted-public-keys = [
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs"
    ];
  };
}
