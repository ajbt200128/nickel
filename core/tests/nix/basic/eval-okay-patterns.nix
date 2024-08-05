[
  (("abcxyzDDDDEFijk") == (
    let

      f = args@{ x, y, z }: x + args.y + z;

      g = { x, y, z }@args: f args;

      # This used to be
      #   h = { x ? "d", y ? x, z ? args.x }@args: x + y + z;
      # But this is hard to transpile, and also somewhat ridiculous
      # So let's not worry about it for now
      h = { x ? "d", y ? x, z ? x }@args: x + y + z;

      j = { x, y, z, ... }: x + y + z;

    in
    f { x = "a"; y = "b"; z = "c"; } +
      g { x = "x"; y = "y"; z = "z"; } +
      h { x = "D"; } +
      h { x = "D"; y = "E"; z = "F"; } +
      j { x = "i"; y = "j"; z = "k"; bla = "bla"; foo = "bar"; }
  ))
]
