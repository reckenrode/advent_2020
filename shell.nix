with import <nixpkgs> {
  overlays = [
    (import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz))
  ];
};
let
  rust_stable = (latest.rustChannels.stable.rust.override {
    extensions = [ "rust-src" ];
  });
in
stdenv.mkDerivation {
  name = "rust-env";
  nativeBuildInputs = with pkgs; [
    rust_stable
    dotnet-sdk_5
  ];
}
