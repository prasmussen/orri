{ pkgs ? import <nixpkgs> {} }:
let
  src =
    ./static;

  cmd =
    ''
    mkdir -p $out
    cp -rf ${src}/* $out/
    '';
in
pkgs.runCommand "orri-frontend" {} cmd
