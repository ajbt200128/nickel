[
  (({ "1" = [ 9 ]; "2" = [ 8 ]; "3" = [ 13 29 ]; "4" = [ 3 4 10 11 17 18 ]; "5" = [ 0 23 26 28 ]; "6" = [ 1 12 21 27 30 ]; "7" = [ 7 22 ]; "8" = [ 14 ]; "9" = [ 19 ]; b = [ 16 25 ]; c = [ 24 ]; d = [ 2 ]; e = [ 5 6 15 31 ]; f = [ 20 ]; }) == (with import ./lib.nix;

  builtins.groupBy
    (n:
      builtins.substring 0 1 (builtins.hashString "sha256" (toString n))
    )
    (range 0 31)))
]