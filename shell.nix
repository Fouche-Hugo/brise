{ pkgs ? import <nixpkgs> {}
}: pkgs.mkShell {
  nativeBuildInputs = with pkgs.buildPackages; [
    cargo
    clippy
    rustfmt
    rustc
    (vscode-with-extensions.override {
      vscodeExtensions = with vscode-extensions; [
        bbenoist.nix
        arrterian.nix-env-selector
        rust-lang.rust-analyzer
        tamasfe.even-better-toml
        vscodevim.vim
      ];
    })
  ];
}