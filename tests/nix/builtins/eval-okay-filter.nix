[
  (([ 0 2 4 6 8 10 100 102 104 106 108 110 ]) == (with import ./lib.nix;

  builtins.filter
    (x: x / 2 * 2 == x)
    (builtins.concatLists [ (range 0 10) (range 100 110) ])))
]
