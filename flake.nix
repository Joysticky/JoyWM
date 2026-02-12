{
  inputs.flakelight.url = "github:nix-community/flakelight";
  outputs = {flakelight, ...}:
    flakelight ./. {
      devShell = {
        packages = pkgs: [
          pkgs.cargo
          pkgs.rust-analyzer
          pkgs.rustc
          pkgs.clippy
          pkgs.rustfmt

          pkgs.libxkbcommon
          pkgs.wayland
          pkgs.wayland-protocols
          pkgs.wayland-scanner
          pkgs.libGL

          pkgs.just
        ];

        shellHook = pkgs: ''
          export RUST_SRC_PATH="${pkgs.rustPlatform.rustLibSrc}";
          export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath [
            pkgs.wayland
            pkgs.libxkbcommon
            pkgs.libGL
          ]}:$LD_LIBRARY_PATH";
        '';
      };
    };
}
