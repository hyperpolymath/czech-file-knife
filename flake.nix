# SPDX-License-Identifier: AGPL-3.0-or-later
# SPDX-FileCopyrightText: 2025 Jonathan D.A. Jewell
#
# czech-file-knife - Nix Flake
# Run: nix develop
# Build: nix build
{
  description = "Czech File Knife - Cloud-native Swiss File Knife for 20+ storage backends";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, crane }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Use stable Rust from overlay
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" ];
        };

        # Crane for building Rust
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        # Common arguments for crane builds
        commonArgs = {
          src = craneLib.cleanCargoSource (craneLib.path ./.);
          strictDeps = true;

          buildInputs = with pkgs; [
            fuse3
            openssl
          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            pkgs.darwin.apple_sdk.frameworks.Security
            pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
            pkgs.libiconv
          ];

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];
        };

        # Build dependencies separately for caching
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        # Build the CLI package
        cfk = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
          cargoExtraArgs = "-p cfk-cli";

          meta = with pkgs.lib; {
            description = "Czech File Knife - Cloud-native Swiss File Knife";
            homepage = "https://github.com/hyperpolymath/czech-file-knife";
            license = licenses.agpl3Plus;
            maintainers = [ ];
            platforms = platforms.all;
            mainProgram = "cfk";
          };
        });

      in
      {
        checks = {
          inherit cfk;

          # Run clippy
          cfk-clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

          # Check formatting
          cfk-fmt = craneLib.cargoFmt {
            src = craneLib.cleanCargoSource (craneLib.path ./.);
          };

          # Run tests
          cfk-test = craneLib.cargoTest (commonArgs // {
            inherit cargoArtifacts;
          });

          # Audit dependencies
          cfk-audit = craneLib.cargoAudit {
            inherit (commonArgs) src;
            advisory-db = pkgs.fetchFromGitHub {
              owner = "rustsec";
              repo = "advisory-db";
              rev = "main";
              sha256 = pkgs.lib.fakeSha256;
            };
          };

          # Deny checks
          cfk-deny = craneLib.cargoDeny {
            inherit (commonArgs) src;
          };
        };

        packages = {
          default = cfk;
          cfk = cfk;
        };

        apps.default = flake-utils.lib.mkApp {
          drv = cfk;
        };

        devShells.default = craneLib.devShell {
          checks = self.checks.${system};

          packages = with pkgs; [
            # Rust toolchain
            rustToolchain

            # Build tools
            pkg-config

            # Development tools
            just
            cargo-watch
            cargo-audit
            cargo-deny
            cargo-outdated
            cargo-machete

            # Testing
            cargo-nextest

            # FUSE support
            fuse3

            # Formatting
            nixpkgs-fmt
          ];

          shellHook = ''
            echo "Czech File Knife development environment"
            echo "Rust: $(rustc --version)"
            echo ""
            echo "Available commands:"
            echo "  just          - Run project tasks"
            echo "  cargo build   - Build the project"
            echo "  cargo test    - Run tests"
            echo "  nix build     - Build with Nix"
            echo "  nix flake check - Run all checks"
          '';
        };
      }
    );
}
