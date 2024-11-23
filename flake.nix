{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-24.05";
  };

  outputs = { self, nixpkgs }:
  let
    # pkgs = nixpkgs.legacyPackages.x86_64-linux;
    pkgs = import nixpkgs {
      system = "x86_64-linux";
      config = {
        android_sdk.accept_license = true; 
        allowUnfree = true;
      };
    };

    androidComposition = pkgs.androidenv.composeAndroidPackages { };
  in
  {

    devShells.x86_64-linux.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          sqlite
          rustfilt
          llvm_17
          just

          nodejs_22

          androidComposition.androidsdk
        ];
    };

  };
}
# run nixos-generate -f sd-aarch64-installer --system armv7l-linux -c sd-image.nix -I nixpkgs="$(pwd)/nixpkgs"
