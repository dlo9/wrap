{
  pkgs,
  lib,
  config,
  inputs,
  ...
}: {
  env = {
    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
    RUST_BACKTRACE = 1;
  };

  git-hooks.hooks = {
    # Nix formatting
    alejandra.enable = true;

    # Check clippy
    clippy.enable = true;
    clippy.settings.denyWarnings = true;

    # Format rust
    rustfmt.enable = true;

    codespell = {
      enable = true;
      name = "Check spelling";
      entry = "codespell";
      extraPackages = with pkgs; [
        codespell
      ];
    };

    cargo-deny = {
      enable = true;
      name = "Lint dependencies";
      entry = "cargo-deny check";
      pass_filenames = false;
      extraPackages = with pkgs; [
        cargo-deny
      ];
    };

    cargo-outdated = {
      enable = true;
      name = "Check for outdated dependencies";
      entry = "cargo outdated -R --exit-code 1";
      pass_filenames = false;
      extraPackages = with pkgs; [
        cargo
        cargo-outdated
      ];
    };

    cargo-build = {
      enable = true;
      name = "Cargo build";
      entry = "cargo build";
      pass_filenames = false;
      extraPackages = with pkgs; [
        cargo
      ];
    };

    cargo-test = {
      enable = true;
      name = "Cargo test";
      entry = "cargo test";
      after = ["cargo-build"];
      pass_filenames = false;
      extraPackages = with pkgs; [
        cargo
      ];
    };
  };

  # Some issues with devenv in vscode:
  # https://github.com/rust-lang/rust-analyzer/issues/15852
  packages = with pkgs; [
    pkg-config
    openssl
    #cargo-udeps # Unused dependencies - requires nightly to run
  ];

  languages.rust = {
    enable = true;
    channel = "stable";
  };
}
