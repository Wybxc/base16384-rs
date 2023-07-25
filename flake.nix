{
  description = "Encode binary files to printable utf16be";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";

    crane = {
      url = "github:ipetkov/crane";
      inputs = {
        flake-utils.follows = "flake-utils";
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs = { self, nixpkgs, fenix, flake-utils, crane, ... }:
    let
      inherit (nixpkgs.lib) optionals;
    in
    flake-utils.lib.eachDefaultSystem (system:
      let
        fenixPkgs = fenix.packages.${system};
        pkgs = nixpkgs.legacyPackages.${system};

        toolchain = fenixPkgs.stable.withComponents [
          "cargo"
          "clippy"
          "rustc"
        ];
        craneLib = crane.lib.${system}.overrideToolchain toolchain;
      in
      let
        commonArgs = {
          src = craneLib.cleanCargoSource (craneLib.path ./.);
          buildInputs = [ ] ++ optionals pkgs.stdenv.isDarwin [
            pkgs.libiconv
          ];
        };
        cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
          pname = "base16384-deps";
        });
        cargoClippy = craneLib.cargoClippy (commonArgs // {
          inherit cargoArtifacts;
          cargoClippyExtraArgs = "--all-targets -- --deny warnings";
        });
        cargoPackage = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        });
      in
      rec {
        packages.default = cargoPackage;
        checks = { inherit cargoPackage cargoClippy; };

        devShells.default = pkgs.mkShell {
          packages = [
            (fenixPkgs.default.withComponents [
              "cargo"
              "clippy"
              "rustc"
              "rustfmt"
            ])
            fenixPkgs.rust-analyzer
          ];

          buildInputs = optionals pkgs.stdenv.isDarwin [
            pkgs.darwin.apple_sdk.frameworks.CoreServices
            pkgs.libiconv
          ];

          RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
        };
      });
}
