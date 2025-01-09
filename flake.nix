{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
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
      in
      {
        devShell = with pkgs; mkShell {
          buildInputs = buildInputs;
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
        packages.default = pkgs.stdenv.mkDerivation {
          SSL_CERT_FILE = "${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt";
          GIT_SSL_CAINFO = "${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt";
          CARGO_HOME = "${placeholder "out"}/.cargo";

          pname = "banj-cli";
          version = "0.0.0";
          src = ./.;
          # run tests?
          doCheck=false;
          inherit buildInputs;
            # Add these:
          buildPhase = ''
            cargo build -r
          '';
          installPhase = ''
            mkdir -p $out/bin
            mv target/release/banjrs $out/bin/banj
          '';
        };
      }
    );
}
