{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.nixpkgs-mozilla.url = "github:mozilla/nixpkgs-mozilla";

  outputs = { self, nixpkgs, nixpkgs-mozilla, ... }: let
    pkgs = nixpkgs.legacyPackages."x86_64-linux";
  in {
    devShells.x86_64-linux.default = with pkgs; mkShell rec {
      name = "dev-env";

      nativeBuildInputs = [
        rustc
        cargo
        rust-analyzer
        (rustfmt.override { asNightly = true; })
        coz
        cargo-flamegraph
        gdb

        llvmPackages_latest.clang
        llvmPackages_latest.lld
        mold

        pkg-config
      ];

      buildInputs = [
        udev alsaLib vulkan-loader 
        xorg.libX11 xorg.libXcursor xorg.libXrandr xorg.libXi
        libxkbcommon wayland

        expat freetype fontconfig
      ];

      LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
    };
  };
}
