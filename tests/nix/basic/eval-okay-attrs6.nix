[
  (({ __overrides = { bar = "qux"; }; bar = "qux"; foo = "bar"; }) == (rec {
    "${"foo"}" = "bar";
    __overrides = { bar = "qux"; };
  }))
]
