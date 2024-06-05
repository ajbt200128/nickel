[
  (({ absolute = /foo; expr = ./foo/bar; home = ~/foo; notfirst = ./bar/foo; simple = ./foo; slashes = /foo/bar; surrounded = ./a-foo-b; }) == (
    let
      foo = "foo";
    in
    {
      simple = ./${foo};
      surrounded = ./a-${foo}-b;
      absolute = /${foo};
      expr = ./${foo + "/bar"};
      home = ~/${foo};
      notfirst = ./bar/${foo};
      slashes = /${foo}/${"bar"};
    }
  ))
]
