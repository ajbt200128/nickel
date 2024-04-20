[
  (([ 1 2 { c = [ ]; d = 4; x = { c = [ ]; }; y = { d = [ ]; }; } { inner = { c = 3; d = 4; }; } ]) == (
    let
      inherit (builtins.trace "used" { a = 1; b = 2; }) a b;
      x.c = 3;
      y.d = [ ];

      merged = {
        inner = {
          inherit (y) d;
        };

        inner = {
          inherit (x) c;
        };
      };
    in
    [ a b rec { x.c = [ ]; inherit (x) c; inherit (y) d; } merged ]
  ))
]
