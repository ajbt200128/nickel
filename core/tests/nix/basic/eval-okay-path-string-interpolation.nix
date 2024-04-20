[
  (({ absolute = /foo; expr = /pwd/lang/foo/bar; home = /fake-home/foo; notfirst = /pwd/lang/bar/foo; simple = /pwd/lang/foo; slashes = /foo/bar; surrounded = /pwd/lang/a-foo-b; }) == (
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
