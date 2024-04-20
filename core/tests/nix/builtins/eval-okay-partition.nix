[
  (({ right = [ 0 2 4 6 8 10 100 102 104 106 108 110 ]; wrong = [ 1 3 5 7 9 101 103 105 107 109 ]; }) == (with import ./lib.nix;

  builtins.partition
    (x: x / 2 * 2 == x)
    (builtins.concatLists [ (range 0 10) (range 100 110) ])))
]
