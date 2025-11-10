{
  description = "Safe disk cleanup CLI/TUI tool";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        rustToolchain = pkgs.rust-bin.stable.latest.default;

        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };

        safe-clean = rustPlatform.buildRustPackage {
          pname = "safe-clean";
          version = "0.1.0";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          meta = with pkgs.lib; {
            description = "Safe disk cleanup CLI/TUI tool";
            homepage = "https://github.com/npsg02/safe-clean";
            license = licenses.mit;
            maintainers = [ ];
            mainProgram = "safe-clean";
          };
        };

      in
      {
        packages = {
          default = safe-clean;
          safe-clean = safe-clean;
        };

        apps = {
          default = {
            type = "app";
            program = "${safe-clean}/bin/safe-clean";
          };
          safe-clean = {
            type = "app";
            program = "${safe-clean}/bin/safe-clean";
          };
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            pkg-config
            cargo-watch
            rust-analyzer
          ];

          shellHook = ''
            echo "Safe-clean development environment"
            echo "Run 'cargo build' to build the project"
            echo "Run 'cargo run' to run the application"
          '';
        };
      }
    ) // {
      # NixOS module
      nixosModules.default = { config, lib, pkgs, ... }:
        with lib;
        let
          cfg = config.programs.safe-clean;
        in
        {
          options.programs.safe-clean = {
            enable = mkEnableOption "safe-clean disk cleanup tool";
          };

          config = mkIf cfg.enable {
            environment.systemPackages = [ self.packages.${pkgs.system}.safe-clean ];
          };
        };
    };
}
