{
  description = "My Advent of Code 2022 solutions in Rust";

  inputs = { 
    nixpkgs.url = "github:nixos/nixpkgs/master";
  };

  outputs = { self, nixpkgs }: let
    pkgs = import nixpkgs {
      system = "x86_64-linux";
    };

  in {
    devShell.x86_64-linux = pkgs.mkShell {
      name = "aoc2022";
      buildInputs = with pkgs; [
        cargo
        rustc
        rust-analyzer
        rustfmt
      ];
      shellHook = ''
      '';
    };
  };
}
