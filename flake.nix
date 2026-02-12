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
        ];

        shellHook = pkgs: ''
          export RUST_SRC_PATH="${pkgs.rustPlatform.rustLibSrc}";
        '';
      };
    };
}
