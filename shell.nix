let
  pkgs = import <nixpkgs> {};
  inherit (pkgs) stdenv;
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  rust_date = "2020-03-06";
  rust_channel = "nightly";
  rust_targets = [ "x86_64-unknown-linux-gnu" ];
  rust_build = nixpkgs.rustChannelOfTargets rust_channel rust_date rust_targets ;
in
pkgs.mkShell {
  buildInputs = [
  rust_build
  ];
  LD_LIBRARY_PATH="${stdenv.cc.cc.lib}/lib64:$LD_LIBRARY_PATH";
}
