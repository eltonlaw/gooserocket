{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system overlays; };
        python = pkgs.python3;
        overlays = [ (import rust-overlay) ];
        rustVersion = pkgs.rust-bin.stable.latest.default;
        gr-python = import ./nix/python.nix { inherit pkgs; };
      in {
        devShell = pkgs.mkShell {
          buildInputs = [
            (python.withPackages gr-python.packages)
            (rustVersion.override { extensions = [ "rust-src" ]; } )
            ]; }; }); }
