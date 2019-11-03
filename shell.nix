with import <nixpkgs> {};

let
  mozilla = callPackage ./nix/mozilla.nix {};
  rustStable = mozilla.latest.rustChannels.stable.rust;
in pkgs.mkShell {
  buildInputs = [
    gcc-arm-embedded
    minicom
    openocd
    (rustStable.override {
      targets = [ "thumbv6m-none-eabi" ];
    })
  ];
}
