{ pkgs ? import <nixpkgs> { } }:
with pkgs;
rec {
  packages = ps: with ps; [
    # biopy
    (buildPythonPackage rec {
       pname = "bio";
       version = "1.5.9";
       format = "wheel";
       src = fetchurl {
         url = "https://files.pythonhosted.org/packages/79/ab/ad10761bdf295a5f2bfdc2734ed61d3c82aa570f0b679428dbe2caa2727c/bio-1.5.9-py3-none-any.whl";
         sha256 = "sha256-scROUJkilVpdh94hX303N5ayeHQXNdhs5WY5ZdXPOGs=";
       };
       propagatedBuildInputs = [ ]; })
    biopython
    ipython
    matplotlib
    pandas
    pip
    numpy
    requests
    tqdm
  ];
}
