{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # Rust toolchain
    rustc
    cargo
    rustfmt
    rust-analyzer
    clippy

    # Additional cargo tools
    cargo-release

    # Build dependencies
    pkg-config
    openssl
  ];

  # Environment variables
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

  shellHook = ''
    echo "Rust development environment loaded"
    echo "Available tools:"
    echo "  - cargo-release for publishing"
    echo "  - Standard Rust toolchain (cargo, rustc, clippy, rustfmt)"
    echo ""
    echo "To publish all crates: cargo release --workspace"
  '';
}