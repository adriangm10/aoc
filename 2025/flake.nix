{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }: 
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in {
        packages.aoc = pkgs.aoc;

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            ocamlPackages.ocamlformat
            ocamlPackages.ocaml-lsp
            ocamlPackages.ocaml
            ocamlPackages.utop
            dune_3
          ];
        };
      });
}
