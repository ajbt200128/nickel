[
  (([ [ 1 3 5 7 9 ] [ "a" "z" "b" "z" ] ]) == (with import ./lib.nix;

  [
    (builtins.concatMap (x: if x / 2 * 2 == x then [ ] else [ x ]) (range 0 10))
    (builtins.concatMap (x: [ x ] ++ [ "z" ]) [ "a" "b" ])
  ]))
]
