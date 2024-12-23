{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-24.05";
  };

  outputs = { self, nixpkgs }:
  let
    pkgs = nixpkgs.legacyPackages.x86_64-linux;
  in
  {

    devShells.x86_64-linux.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          sqlite
          rustfilt
          llvm_17
          just
        ];
    };

  };
}
# run nixos-generate -f sd-aarch64-installer --system armv7l-linux -c sd-image.nix -I nixpkgs="$(pwd)/nixpkgs"
