{
  description = "TUI music player client for OpenSubsonic-compatible servers";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {self, ...} @ inputs: let
    supportedSystems = [
      "x86_64-linux"
      "aarch64-linux"
      "x86_64-darwin"
      "aarch64-darwin"
    ];
    forEachSupportedSystem = f:
      inputs.nixpkgs.lib.genAttrs supportedSystems (
        system:
          f {
            pkgs = import inputs.nixpkgs {
              inherit system;
              overlays = [
                inputs.self.overlays.default
              ];
            };
          }
      );
  in {
    overlays.default = final: prev: {
      rustToolchain = with inputs.fenix.packages.${prev.stdenv.hostPlatform.system};
        combine (
          with stable; [
            clippy
            rustc
            cargo
            rustfmt
            rust-src
          ]
        );
    };

    devShells = forEachSupportedSystem (
      {pkgs}: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            rustToolchain
            openssl
            pkg-config
            cargo-deny
            cargo-edit
            cargo-watch
            rust-analyzer
            llvmPackages.libclang
            llvmPackages.clang
          ];

          env = {
            # Required by rust-analyzer
            RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";
            LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
            LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath [ pkgs.stdenv.cc.cc.lib ]}";
          };
        };
      }
    );
    packages = forEachSupportedSystem (
      {pkgs}: {
        default = let
          naersk-lib = pkgs.callPackage inputs.naersk {};
        in
          naersk-lib.buildPackage {
            src = ./.;
            nativeBuildInputs = with pkgs; [pkg-config clang];
            buildInputs = with pkgs; [openssl llvmPackages.libclang];
            LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";

            postInstall = ''
              mkdir -p $out/lib
              cp sdk/lib/libdev.so $out/lib/
              # Add $out/lib and libstdc++ to RPATH
              patchelf --add-rpath $out/lib:${pkgs.stdenv.cc.cc.lib}/lib $out/bin/obsbot-cli
            '';
          };
      }
    );
  };
}
