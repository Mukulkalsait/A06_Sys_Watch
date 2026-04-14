{
  description = "Syswatch - Rust System Monetoring Tool For servers, with Nix Deployment.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: {
    let system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };
    in{

    # DevShell
    devShell.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        rustc
        cargo
        rustfmt
        clippy
      ];
    };

    # Build pkgs
    packages.${system}.default = pkgs.rustPlatform.buildRustPackages {
      pname = "syswatch";
      version = "0.1.0";
      src = ./.;
      cargoLock = {
        locaFile = ./Cargo.lock
          };

      };
    };

  };
}
