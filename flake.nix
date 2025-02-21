{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, fenix }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        buildInputs = with pkgs; [ 
          cargo 
          rustc 
          rustfmt 
          rustPackages.clippy 
          pkg-config 
          gcc 
          openssl 
          cacert
          openssl.dev
        ];
        pkgs = nixpkgs.legacyPackages.${system};
        rustPlatform = pkgs.makeRustPlatform {
         cargo = fenix.packages.${system}.minimal.toolchain;
          rustc = fenix.packages.${system}.minimal.toolchain;
        };
      in
      {
        devShell = with pkgs; mkShell {
          buildInputs = buildInputs;
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
        packages.default = rustPlatform.buildRustPackage rec {
          pname = "banj-cli";
          version = "0.1.0";

          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
        };
      }
    );
}
