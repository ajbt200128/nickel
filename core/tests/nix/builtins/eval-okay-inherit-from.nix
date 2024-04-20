[
  (([ 1 2 { __overrides = { y = { d = [ ]; }; }; c = [ ]; d = 4; x = { c = [ ]; }; y = «repeated»; } { inner = { c = 3; d = 4; }; } ]) == (
    let
      inherit (builtins.trace "used" { a = 1; b = 2; }) a b;
      x.c = 3;
      y.d = 4;

      merged = {
        inner = {
          inherit (y) d;
        };

        inner = {
          inherit (x) c;
        };
      };
    in
    [ a b rec { x.c = [ ]; inherit (x) c; inherit (y) d; __overrides.y.d = [ ]; } merged ]
  ))
]
