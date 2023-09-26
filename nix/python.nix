{ pkgs ? import <nixpkgs> { } }:
with pkgs;
rec {
  packages = ps: with ps; [
    # biopy
    ipython
    matplotlib
    pandas
    numpy
    requests
  ];
}
