let
  # As of Winter 2020, .NET 5.0 is only available in the unstable channel
  pkgs = import (fetchTarball https://nixos.org/channels/nixos-unstable/nixexprs.tar.xz) {};
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    dotnet-sdk_5
  ];
}
