[
  (({ "0" = { n = "0"; v = [ 5 23 29 ]; }; "1" = { n = "1"; v = [ 7 30 ]; }; "2" = { n = "2"; v = [ 18 ]; }; "4" = { n = "4"; v = [ 10 ]; }; "5" = { n = "5"; v = [ 15 25 26 31 ]; }; "6" = { n = "6"; v = [ 3 14 ]; }; "7" = { n = "7"; v = [ 12 ]; }; "8" = { n = "8"; v = [ 2 6 8 9 ]; }; "9" = { n = "9"; v = [ 0 16 ]; }; a = { n = "a"; v = [ 17 21 22 27 ]; }; c = { n = "c"; v = [ 11 24 ]; }; d = { n = "d"; v = [ 4 13 28 ]; }; e = { n = "e"; v = [ 20 ]; }; f = { n = "f"; v = [ 1 19 ]; }; }) == (with import ./lib.nix;

  let
    str = builtins.hashString "sha256" "test";
  in
  builtins.zipAttrsWith
    (n: v: { inherit n v; })
    (map (n: { ${builtins.substring n 1 str} = n; })
      (range 0 31))))
]
