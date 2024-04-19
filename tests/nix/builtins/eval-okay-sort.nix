[
  (([ [ 42 77 147 249 483 526 ] [ 526 483 249 147 77 42 ] [ "bar" "fnord" "foo" "xyzzy" ] [{ key = 1; value = "foo"; } { key = 1; value = "fnord"; } { key = 2; value = "bar"; }] [ [ ] [ ] [ 1 ] [ 1 4 ] [ 1 5 ] [ 1 6 ] [ 2 ] [ 2 3 ] [ 3 ] [ 3 ] ] ]) == (with builtins;

  [
    (sort lessThan [ 483 249 526 147 42 77 ])
    (sort (x: y: y < x) [ 483 249 526 147 42 77 ])
    (sort lessThan [ "foo" "bar" "xyzzy" "fnord" ])
    (sort (x: y: x.key < y.key)
      [{ key = 1; value = "foo"; } { key = 2; value = "bar"; } { key = 1; value = "fnord"; }])
    (sort lessThan [
      [ 1 6 ]
      [ ]
      [ 2 3 ]
      [ 3 ]
      [ 1 5 ]
      [ 2 ]
      [ 1 ]
      [ ]
      [ 1 4 ]
      [ 3 ]
    ])
  ]))
]
