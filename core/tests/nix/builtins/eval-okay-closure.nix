[
  (([{ foo = true; key = -13; } { foo = true; key = -12; } { foo = true; key = -11; } { foo = true; key = -9; } { foo = true; key = -8; } { foo = true; key = -7; } { foo = true; key = -5; } { foo = true; key = -4; } { foo = true; key = -3; } { key = -1; } { foo = true; key = 0; } { foo = true; key = 1; } { foo = true; key = 2; } { foo = true; key = 4; } { foo = true; key = 5; } { foo = true; key = 6; } { key = 8; } { foo = true; key = 9; } { foo = true; key = 10; } { foo = true; key = 13; } { foo = true; key = 14; } { foo = true; key = 15; } { key = 17; } { foo = true; key = 18; } { foo = true; key = 19; } { foo = true; key = 22; } { foo = true; key = 23; } { key = 26; } { foo = true; key = 27; } { foo = true; key = 28; } { foo = true; key = 31; } { foo = true; key = 32; } { key = 35; } { foo = true; key = 36; } { foo = true; key = 40; } { foo = true; key = 41; } { key = 44; } { foo = true; key = 45; } { foo = true; key = 49; } { key = 53; } { foo = true; key = 54; } { foo = true; key = 58; } { key = 62; } { foo = true; key = 67; } { key = 71; } { key = 80; }]) == (
    let

      closure = builtins.genericClosure {
        startSet = [{ key = 80; }];
        operator = { key, foo ? false }:
          if builtins.lessThan key 0
          then [ ]
          else [{ key = builtins.sub key 9; } { key = builtins.sub key 13; foo = true; }];
      };

      sort = (import ./lib.nix).sortBy (a: b: builtins.lessThan a.key b.key);

    in
    sort closure
  ))
]