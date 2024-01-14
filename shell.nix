{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    buildInputs = with pkgs; [
      openssl
    ];
    nativeBuildInputs = with pkgs.buildPackages; [ rustc cargo pkg-config ];
}
