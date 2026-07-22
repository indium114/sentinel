{
  description = "rust devshell and package, created by scaffolder";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk = {
      url ="github:nix-community/naersk";
      inputs.nixpkgs.url = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      naersk,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };

        naersk' = pkgs.callPackage naersk {};
      in
      {
        devShells.default = pkgs.mkShell {
          name = "rust-devshell";

          packages = with pkgs; [
            cargo
            rustc
            rustfmt
            rust-analyzer
            clippy
            pkg-config
            lua54Packages.lua
          ];
        };

        packages.sentinel = naersk'.buildPackage {
          src = ./.;

          nativeBuildInputs = [
            pkgs.pkg-config
          ];

          buildInputs = [
            pkgs.lua54Packages.lua
          ];
        };

        apps.sentinel = {
          type = "app";
          program = "${self.packages.${pkgs.stdenv.hostPlatform.system}.sentinel}/bin/sentinel";
        };
      }
    );
}
