{ compiler ? "ghc881", pkgs ? import <nixpkgs> {} }:

let
  haskellPackages = pkgs.haskell.packages.${compiler};
  drv = haskellPackages.callCabal2nix "aoc2019" ./. {};
in
  {
    aoc = drv;
    aoc-shell = haskellPackages.shellFor {
      packages = p: [drv];
      buildInputs = with pkgs; [ cabal-install hlint ];
    };
  }
