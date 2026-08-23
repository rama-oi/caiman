{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  # xkbcommon (the crate) links against libxkbcommon for keysym name <->
  # character conversion. `xkbcli` (same package) is also invoked as a
  # subprocess at runtime to list layouts and compile keymaps.
  nativeBuildInputs = [
    pkgs.pkg-config
  ];

  buildInputs = [
    pkgs.libxkbcommon
  ];
}