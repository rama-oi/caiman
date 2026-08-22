{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  # `xkbcli` (shipped with libxkbcommon) is invoked as a subprocess at
  # runtime to list layouts and compile keymaps — caiman no longer links
  # against libxkbcommon directly, so this is just here to put `xkbcli` on
  # PATH.
  packages = [
    pkgs.libxkbcommon
  ];
}