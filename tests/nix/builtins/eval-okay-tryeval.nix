[
  (({ x = { success = true; value = "x"; }; y = { success = false; value = false; }; z = { success = false; value = false; }; }) == ({
    x = builtins.tryEval "x";
    y = builtins.tryEval (assert false; "y");
    z = builtins.tryEval (throw "bla");
  }))
]
