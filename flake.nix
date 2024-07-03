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
          config.allowUnfree = true;
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
            # missing: bioawk csvtk bedgraphtobigwig
            pkgs.bcftools
            pkgs.bedtools
            pkgs.bwa
            pkgs.bowtie2
            pkgs.datamash
            pkgs.emboss
            pkgs.emboss
            # pkgs.fastqr
            pkgs.fastp
            pkgs.freebayes
            pkgs.hisat2
            pkgs.mafft
            pkgs.nix-prefetch
            pkgs.parallel
            pkgs.samtools
            pkgs.seqkit
            pkgs.snpeff
            pkgs.subread
            pkgs.sratoolkit
            pkgs.trimmomatic
            pkgs.wget
            ]; }; }); }
