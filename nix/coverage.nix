{
  ...
}:

{
  perSystem =
    {
      pkgs,
      crane,
      rust,
      ...
    }:
    let
      mkToolchain =
        pkgs:
        let
          prev = rust.mkToolchain pkgs;
        in
        prev.override {
          extensions = (prev.extensions or [ ]) ++ [
            # Needed by cargo-llvm-cov to generate coverage.
            "llvm-tools-preview"
          ];
        };
    in
    {
      packages.coverage = (crane.lib.overrideToolchain mkToolchain).cargoLlvmCov (
        crane.commonArgs
        // {
          buildPhaseCargoCommand = ''
            # Run unit tests.
            cargo llvm-cov --no-report --workspace

            # Run integration tests.
            cargo llvm-cov --no-report --package=tests --features=auth,collab,mock,walkdir

            # Generate coverage report.
            cargo llvm-cov report --codecov --output-path codecov.json
          '';
          installPhaseCommand = ''
            mkdir -p $out
            mv codecov.json $out/
          '';
        }
      );

      ciDevShells.coverage = {
        packages = [
          (mkToolchain pkgs)
          pkgs.cargo-llvm-cov
        ];
      };
    };
}
