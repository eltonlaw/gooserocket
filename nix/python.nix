{ pkgs ? import <nixpkgs> {} }:
with pkgs;
rec {
  packages = ps: with ps; [
    (buildPythonPackage rec {
       pname = "bio";
       version = "1.5.9";
       format = "wheel";
       src = fetchurl {
         url = "https://files.pythonhosted.org/packages/c1/1e/6c4808999f33f63ed1327c6463d03f1209b08634183a237d9112370071b4/bio-1.6.2-py3-none-any.whl";
         sha256 = "dcec8207f3993a7f41bd8205cde235218da3dc008175ae7bb7468276d8dacf71";
       };
       propagatedBuildInputs = [ ]; })
    # biopython
    ipython
    matplotlib
    pandas
    pip
    numpy
    requests
    tqdm
  ];
}
