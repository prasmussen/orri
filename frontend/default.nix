{ pkgs ? import <nixpkgs> {} }:
let
  src =
    ./static;

  cmd =
    ''
    mkdir -p $out/static
    cp -rf ${src}/* $out/static
    '';
in
pkgs.runCommand "orri-frontend" {} cmd
