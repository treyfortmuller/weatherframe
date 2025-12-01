{
  description = "A simple flake-based rust project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
    }:

    flake-utils.lib.eachSystem [ "aarch64-linux" "x86_64-linux" ] (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        rust = pkgs.rust-bin.stable.latest.default.override {
          # rust-src is required for ctrl+clicking into standard library functions using RUST_SRC_PATH
          # rust-bin from oxalica is pre-built binary distribution and doesn't include src by default.
          extensions = [ "rust-src" ];
          targets = [ ]; # e.g. "thumbv7em-none-eabihf"
        };

        # Create a rustPlatform using oxalica's toolchain
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rust;
          rustc = rust;
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rust
            pkg-config
            openssl

            # TODO (tff): may or may not end up using this as our rendering engine
            wkhtmltopdf
          ];

          # Optional, useful sometimes
          # RUST_BACKTRACE = "1";

          # Ctrl+click on the standard library
          RUST_SRC_PATH = "${rust}/lib/rustlib/src/rust";

          shellHook = ''
            echo "🦀 Evolved into a crab again... shit."
            rustc --version
          '';
        };

        packages.default =
          let
            # Read the package name and the crate version info from the Cargo.toml
            cargoToml = builtins.fromTOML (builtins.readFile "${self}/Cargo.toml");
            crateName = cargoToml.package.name;
            crateVersion = cargoToml.package.version;
          in
          rustPlatform.buildRustPackage {
            pname = crateName;
            version = crateVersion;
            src = self;

            cargoLock = {
              lockFile = "${self}/Cargo.lock";

              # Nix needs inputs to be content-addressable and git dependencies are not, even for fixed revs in
              # your Cargo.toml so we need to specify these.
              outputHashes = {
                "libtatted-0.1.0" = "sha256-lT3NI/VJArANsZ12fAjCIF13sBjQuLoRkVuAeyhHGYA=";
                "openwx-0.1.0" = "sha256-HA+B4J3MQ3l+bfOLJKAwDOZrBH6XsYaqFwdn7UTUoS0=";
              };
            };

            nativeBuildInputs = [ pkgs.pkg-config ];
            buildInputs = [ pkgs.openssl ];
          };

        formatter = pkgs.nixfmt-tree;
      }
    );
}
