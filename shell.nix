{ pkgs ? import <unstable> { } }:
pkgs.mkShell {
  buildInputs = with pkgs; [ cmake pkg-config freetype expat fontconfig ];
  shellHook = ''
  '';
}
