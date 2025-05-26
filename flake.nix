{
  description = "A small utility to wrap other commands. Like `alias`, but better.";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [
        inputs.flake-parts.flakeModules.easyOverlay
      ];

      systems = ["x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin"];

      perSystem = {
        config,
        self',
        inputs',
        pkgs,
        system,
        ...
      }: {
        overlayAttrs = {
          inherit (config.packages) wrap;
        };

        packages = rec {
          default = config.packages.wrap;

          wrap = let
            cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          in
            pkgs.rustPlatform.buildRustPackage {
              pname = "wrap";
              version = cargoToml.package.version;

              src = ./.;
              release = true;

              cargoLock = {
                lockFile = ./Cargo.lock;
              };

              meta = with pkgs.lib; {
                description = "A small utility to wrap other commands. Like `alias`, but better.";
                homepage = "https://github.com/dlo9/wrap";
                license = cargoToml.package.license;
              };
            };
        };
      };
    };
}
