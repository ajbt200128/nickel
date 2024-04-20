[
  (([ 5 4 "int" "tt" "float" 4 ]) == ([
    (builtins.add 2 3)
    (builtins.add 2 2)
    (builtins.typeOf (builtins.add 2 2))
    ("t" + "t")
    (builtins.typeOf (builtins.add 2.0 2))
    (builtins.add 2.0 2)
  ]))
]
