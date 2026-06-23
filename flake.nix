{
  description = "rust devshell and package, created by scaffolder";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in {
        devShells.default = pkgs.mkShell {
          name = "rust-devshell";

          packages = with pkgs; [
            cargo
            rustc
            rustfmt
            rust-analyzer
            clippy
            pkg-config
            lua55Packages.lua
          ];
        };

        packages.sentinel = pkgs.rustPlatform.buildRustPackage {
          name = "sentinel";
          version = "0.1.0";

          src = ./.;

          cargoLock.lockFile = ./Cargo.lock;
        };

        apps.sentinel = {
          type = "app";
          program = "${self.packages.${pkgs.stdenv.hostPlatform.system}.sentinel}/bin/sentinel";
        };
      });
}
