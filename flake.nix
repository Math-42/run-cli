{
  description = "A run-codes cli front end with some extra features";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-22.05";
    flake-utils.url = "github:numtide/flake-utils";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };

  outputs = { self, nixpkgs, flake-utils, crane }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        craneLib = crane.lib.${system};
      in
      rec {
        packages.default = craneLib.buildPackage {
          src = craneLib.cleanCargoSource ./.;
          buildInputs = [ pkgs.openssl ];
          nativeBuildInputs = [ pkgs.pkg-config ];
        };
        devShell = pkgs.mkShell {
          inputsFrom = [ packages.default ];
          buildInputs = [
            pkgs.clippy # Linter
            pkgs.rustfmt # Formatter
            pkgs.rust-analyzer # LSP
          ];
        };
      });
}
