{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    polkadot.url = "github:andresilva/polkadot.nix";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      polkadot,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [
          (import rust-overlay)
          polkadot.overlays.default
        ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default = pkgs.mkShell {
          packages =
            with pkgs;
            [
              (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
              cargo-leptos
              clang
              pkg-config
              openssl
              dart-sass
              binaryen
              wasm-bindgen-cli
              websocat
              subxt
              nodejs
              nodePackages.pnpm
              just
              leptosfmt
            ]
            ++ lib.optionals stdenv.hostPlatform.isLinux [ rust-jemalloc-sys-unprefixed ]
            ++ lib.optionals stdenv.hostPlatform.isDarwin [
              darwin.apple_sdk.frameworks.Security
              darwin.apple_sdk.frameworks.SystemConfiguration
            ];
        };
      }
    );
}
