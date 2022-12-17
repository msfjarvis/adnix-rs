{
  description = "adnix";

  inputs = {
    nixpkgs = { url = "github:NixOS/nixpkgs/nixpkgs-unstable"; };

    flake-utils = { url = "github:numtide/flake-utils"; };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };

    crane = {
      url = "github:ipetkov/crane";
      inputs = {
        flake-utils.follows = "flake-utils";
        nixpkgs.follows = "nixpkgs";
        rust-overlay.follows = "rust-overlay";
      };
    };

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs =
    { self, nixpkgs, crane, flake-utils, advisory-db, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rustStable =
          pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        craneLib = (crane.mkLib pkgs).overrideToolchain rustStable;
        src = ./.;
        cargoArtifacts = craneLib.buildDepsOnly { inherit src buildInputs; };
        buildInputs = [ ];

        adnix = craneLib.buildPackage {
          inherit src;
          doCheck = false;
        };
        adnix-clippy = craneLib.cargoClippy {
          inherit cargoArtifacts src buildInputs;
          cargoClippyExtraArgs = "--all-targets -- --deny warnings";
        };
        adnix-fmt = craneLib.cargoFmt { inherit src; };
        adnix-audit = craneLib.cargoAudit { inherit src advisory-db; };
        adnix-nextest = craneLib.cargoNextest {
          inherit cargoArtifacts src buildInputs;
          partitions = 1;
          partitionType = "count";
        };
      in {
        checks = {
          inherit adnix adnix-audit adnix-clippy adnix-fmt adnix-nextest;
        };

        packages.default = adnix;

        apps.default = flake-utils.lib.mkApp { drv = adnix; };

        devShells.default = pkgs.mkShell {
          inputsFrom = builtins.attrValues self.checks;

          nativeBuildInputs = with pkgs; [
            cargo-nextest
            cargo-release
            rustStable
          ];
        };
      });
}
