{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = inputs @ { 
    self, 
    fenix, 
    nixpkgs,
    ...
  }: let
    inherit (nixpkgs) lib;
    genSystems = lib.genAttrs [
      "aarch64-linux"
      "x86_64-linux"
    ];
    pkgsFor = nixpkgs.legacyPackages;
  in {
    devShells = genSystems (system: let
      pkgs = pkgsFor.${system};
      fenixpkgs = fenix.packages.${system};
    in {
      default = let 
        toolchain = fenixpkgs.complete.withComponents [
          "cargo"
          "clippy"
          "rust-src"
          "rustc"
          "rustfmt"
        ];
      in pkgsFor.${system}.mkShell rec {
        name = "kubia-timer-env";
        nativeBuildInputs = with pkgs; [
          toolchain
          fenix.packages.${system}.rust-analyzer
          pkg-config
          cmake
        ];
        buildInputs = with pkgs; [
          fontconfig
        ];
        shellHook = ''
          export RUST_SRC_PATH="${toolchain}/lib/rustlib/src/rust/library"
          export RUST_BACKTRACE=1
          export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:\
          ${pkgs.xorg.libX11}/lib:\
          ${pkgs.xorg.libXcursor}/lib:\
          ${pkgs.xorg.libXrandr}/lib:\
          ${pkgs.xorg.libXi}/lib:\
          ${pkgs.libxkbcommon}/lib:\
          ${pkgs.vulkan-loader}/lib:\
          ${pkgs.wayland}/lib:\
          ${pkgs.libGL}/lib"
          export NIX_SHELL_ACTIVE=${name}
        '';
      };
    });
  };
}
