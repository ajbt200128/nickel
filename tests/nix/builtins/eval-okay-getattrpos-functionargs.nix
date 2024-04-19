[
  (({ column = 11; file = "eval-okay-getattrpos-functionargs.nix"; line = 2; }) == (
    let
      fun = { foo }: { };
      pos = builtins.unsafeGetAttrPos "foo" (builtins.functionArgs fun);
    in
    { inherit (pos) column line; file = baseNameOf pos.file; }
  ))
]
