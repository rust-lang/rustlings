{
  description = "Small exercises to get you used to reading and writing Rust code";

  inputs = {
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, flake-utils, nixpkgs, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        rustlings =
          pkgs.rustPlatform.buildRustPackage {
            name = "rustlings";
            version = "5.3.0";

            src = with pkgs.lib; cleanSourceWith {
              src = self;
              # a function that returns a bool determining if the path should be included in the cleaned source
              filter = path: type:
                let
                  # filename
                  baseName = builtins.baseNameOf (toString path);
                  # path from root directory
                  path' = builtins.replaceStrings [ "${self}/" ] [ "" ] path;
                  # checks if path is in the directory
                  inDirectory = directory: hasPrefix directory path';
                in
                inDirectory "src" ||
                inDirectory "tests" ||
                hasPrefix "Cargo" baseName ||
                baseName == "info.toml";
            };

            cargoLock.lockFile = ./Cargo.lock;
          };
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            rustc
            rust-analyzer
            rustlings
          ];
        };
      });
}
