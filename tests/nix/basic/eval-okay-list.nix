[
  (("foobarblatest") == (with import ./lib.nix;

  let

    body = concat [ "foo" "bar" "bla" "test" ];

  in
  body
  ))
]
