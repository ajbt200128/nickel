[
  ((true) == (builtins.pathExists (./lib.nix)
    && builtins.pathExists (builtins.toPath ./lib.nix)
    && builtins.pathExists (builtins.toString ./lib.nix)
    && !builtins.pathExists (builtins.toString ./lib.nix + "/")
    && !builtins.pathExists (builtins.toString ./lib.nix + "/.")
    # FIXME
    # && !builtins.pathExists (builtins.toString ./lib.nix + "/..")
    # && !builtins.pathExists (builtins.toString ./lib.nix + "/a/..")
    # && !builtins.pathExists (builtins.toString ./lib.nix + "/../lib.nix")
    && !builtins.pathExists (builtins.toString ./lib.nix + "/./")
    && !builtins.pathExists (builtins.toString ./lib.nix + "/./.")
    && builtins.pathExists (builtins.toString ./.. + "/lang/lib.nix")
    && !builtins.pathExists (builtins.toString ./.. + "lang/lib.nix")
    && builtins.pathExists (builtins.toString ./. + "/../lang/lib.nix")
    && builtins.pathExists (builtins.toString ./. + "/../lang/./lib.nix")
    && builtins.pathExists (builtins.toString ./.)
    && builtins.pathExists (builtins.toString ./. + "/")
    && builtins.pathExists (builtins.toString ./. + "/../lang")
    && builtins.pathExists (builtins.toString ./. + "/../lang/")
    && builtins.pathExists (builtins.toString ./. + "/../lang/.")
    && builtins.pathExists (builtins.toString ./. + "/../lang/./")
    && builtins.pathExists (builtins.toString ./. + "/../lang//./")
    && builtins.pathExists (builtins.toString ./. + "/../lang/..")
    && builtins.pathExists (builtins.toString ./. + "/../lang/../")
    && builtins.pathExists (builtins.toString ./. + "/../lang/..//")
    && builtins.pathExists (builtins.toPath (builtins.toString ./lib.nix))
    && !builtins.pathExists (builtins.toPath (builtins.toString ./bla.nix))
    && builtins.pathExists (builtins.toPath { __toString = x: builtins.toString ./lib.nix; })
    && builtins.pathExists (builtins.toPath { outPath = builtins.toString ./lib.nix; })
    && builtins.pathExists ./lib.nix
    && !builtins.pathExists ./bla.nix
    && builtins.pathExists ./symlink-resolution/foo/overlays/overlay.nix
    && builtins.pathExists ./symlink-resolution/broken
    && builtins.pathExists (builtins.toString ./symlink-resolution/foo/overlays + "/.")))
]
