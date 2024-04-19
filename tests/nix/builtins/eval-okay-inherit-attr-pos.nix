[
  (([{ column = 17; file = "/pwd/lang/eval-okay-inherit-attr-pos.nix"; line = 4; } { column = 19; file = "/pwd/lang/eval-okay-inherit-attr-pos.nix"; line = 4; } { column = 21; file = "/pwd/lang/eval-okay-inherit-attr-pos.nix"; line = 5; } { column = 23; file = "/pwd/lang/eval-okay-inherit-attr-pos.nix"; line = 5; }]) == (
    let
      d = 0;
      x = 1;
      y = { inherit d x; };
      z = { inherit (y) d x; };
    in
    [
      (builtins.unsafeGetAttrPos "d" y)
      (builtins.unsafeGetAttrPos "x" y)
      (builtins.unsafeGetAttrPos "d" z)
      (builtins.unsafeGetAttrPos "x" z)
    ]
  ))
]
