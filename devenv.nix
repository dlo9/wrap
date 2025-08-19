{
  pkgs,
  lib,
  config,
  inputs,
  ...
}: {
  cachix.enable = false;

  env = {
    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
    RUST_BACKTRACE = 1;
  };

  # Some issues with devenv in vscode:
  # https://github.com/rust-lang/rust-analyzer/issues/15852
  packages = with pkgs; [
    pkg-config
    openssl
    just
    codespell
    cargo
    cargo-outdated
    cargo-audit
    cargo-deny
    cargo-udeps # Unused dependencies - requires nightly to run
  ];

  languages.rust = {
    enable = true;
    channel = "stable";
  };
}
