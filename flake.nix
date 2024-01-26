{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }: 
  let
    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };
  in utils.lib.eachDefaultSystem (system: 
    let 
      rustPkg = pkgs.rustPlatform.buildRustPackage {
        pname = "splitwise_exporter";
        version = "0.1.0";
        src = ./.;
        buildInputs = with pkgs; [
          pkg-config
          openssl
        ];
        nativeBuildInputs = with pkgs; [ pkg-config ];
        cargoSha256 = "sha256-7esv0lYiYrwDu+27KP0Vh9FF+Cwdk/vKBVq/HjAdAmg=";
      };
    in {
    devShells.default = pkgs.mkShell {
      packages = with pkgs; [ cargo rustc pkg-config openssl ];
    };
    packages.splitwise-exporter = rustPkg;
    defaultPackage = rustPkg;
  });
}
