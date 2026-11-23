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
        gst_all_1.gstreamer
        gst_all_1.gst-plugins-base
        gst_all_1.gst-plugins-good
        gst_all_1.gst-plugins-bad
        gst_all_1.gst-plugins-ugly
      ];
      shellHook = ''
        export PATH="$HOME/.cargo/bin:$PATH"
        export GIO_MODULE_DIR="${pkgs.glib-networking}/lib/gio/modules"
        export GST_PLUGIN_SYSTEM_PATH_1_0="${pkgs.gst_all_1.gst-plugins-base}/lib/gstreamer-1.0:${pkgs.gst_all_1.gst-plugins-good}/lib/gstreamer-1.0:${pkgs.gst_all_1.gst-plugins-bad}/lib/gstreamer-1.0:${pkgs.gst_all_1.gst-plugins-ugly}/lib/gstreamer-1.0"
        export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath (with pkgs; [ gtk3 webkitgtk_4_1 glib glib-networking libsoup_3 openssl gst_all_1.gstreamer gst_all_1.gst-plugins-base gst_all_1.gst-plugins-good ])}:$LD_LIBRARY_PATH"
      '';
    };
  };
}
