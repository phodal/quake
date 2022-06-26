{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    pkg-config
    openssl
    glib
    gdk-pixbuf
    libsoup
    gtk3
    webkitgtk
    libappindicator
  ];
}
