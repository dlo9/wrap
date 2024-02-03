{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {self, ...} @ inputs:
    inputs.flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import inputs.rust-overlay)];
        pkgs = import inputs.nixpkgs {
          inherit system overlays;
        };
        rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      in rec {
        packages = rec {
          # Build with `nix build`
          default = wrap;

          wrap = let
            cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          in
            # TODO: this doesn't match the developer cargo version
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
          buildInputs = with pkgs; [
            rust
            cargo-deny
            codespell
          ];

          shellHook = ''
            # Setup git hooks
            ln -srf hooks/* .git/hooks/
          '';

          RUST_BACKTRACE = 1;
        };

        formatter = pkgs.alejandra;
      }
    );
}
