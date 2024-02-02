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

          wrap = pkgs.rustPlatform.buildRustPackage {
            pname = "wrap";
            version = "0.3.9";

            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
              outputHashes = {
                "config-0.11.0" = "sha256-UDsMWPgmIS2mdfsKiYm1TBXW8lC4pUNCuoz17h5SEOI=";
              };
            };

            meta = with pkgs.lib; {
              description = "A small utility to wrap other commands. Like `alias`, but better.";
              homepage = "https://github.com/dlo9/wrap";
              license = licenses.mit;
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
          nativeBuildInputs = [
            pkgs.cargo
          ];

          shellHook = "code .";

          RUST_BACKTRACE = 1;
        };
      }
    );
}
