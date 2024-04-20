[
  (({ bar = "regular"; foo = "directory"; ldir = "symlink"; linked = "symlink"; }) == (builtins.readDir ./readDir))
]
