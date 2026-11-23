{
  description = "Shoclaw Browser Development Environment";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: let
    pkgs = import nixpkgs {
      system = "x86_64-linux";
      config.allowUnfree = true;
    };
  in {
    devShells.x86_64-linux.default = pkgs.mkShell {
      nativeBuildInputs = with pkgs; [
        pkg-config
        rustup
      ];
      buildInputs = with pkgs; [
        gtk3
        webkitgtk_4_1
        glib
        glib-networking
        libsoup_3
        openssl
      ];
      shellHook = ''
        export PATH="$HOME/.cargo/bin:$PATH"
        export GIO_MODULE_DIR="${pkgs.glib-networking}/lib/gio/modules"
        export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath (with pkgs; [ gtk3 webkitgtk_4_1 glib glib-networking libsoup_3 openssl ])}:$LD_LIBRARY_PATH"
      '';
    };
  };
}
