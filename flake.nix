{
  description = "SysWatch - Quick System Monetoring tool for servers build with Rust + Nix Deployment ";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = { self, nixpkgs, fenix }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ fenix.overlays.default ];
      };
      rustToolchain = pkgs.fenix.complete.toolchain;
    in
    {
      # Devshell 
      devShells.${system}.default =
        pkgs.mkShell {
          buildInputs = with pkgs; [ rustToolchain ];
          shellHook = '' echo "🚀 SysWatch DevShell (FenixRust)" '';
        };

      packages.${system} = {

        # build Pkgs
        default =
          pkgs.rustPlatform.buildRustPackage {
            pname = "SysWatch";
            version = "0.1.0";
            src = ./.;
            cargoLock = {
              lockFile = ./Cargo.lock;
            };

          };

        # DockerTools
        dockerImage =
          pkgs.dockerTools.buildImage
            {
              name = "SysWatch";
              tag = "latest";

              copyToRoot = pkgs.buildEnv {
                name = "image-root";
                paths = [ self.packages.${system}.default ];
                pathsToLink = [ "/bin" ];
              };
              # contents = [ self.packages.${system}.default ];
              # config = { Cmd = [ "${self.packages.${system}.default}/bin/syswatch" ]; };

              config =
                {
                  Cmd = [ "syswatch" ];
                };

            };
      };

    };
}
