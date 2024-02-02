{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  } @ inputs:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
      in rec {
        packages = rec {
          # Build with `nix build`
          default = wrap;

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

        apps = rec {
          # Run with `nix run`
          apps.default = wrap;

          wrap = {
            type = "app";
            program = "${packages.default}/bin/wrap";
          };
        };

        # Enter with `nix develop`
        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.cargo
            pkgs.cargo-deny
          ];

          RUST_BACKTRACE = 1;
        };

        formatter = nixpkgs.legacyPackages.${system}.alejandra;
      }
    );
}
