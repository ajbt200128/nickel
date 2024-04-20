[
  (([ 1 2 3 4 5 6 7 8 9 10 ]) == (
    let

      overrides = {
        import = fn: scopedImport overrides fn;

        scopedImport = attrs: fn: scopedImport (overrides // attrs) fn;

        builtins = builtins // overrides;
      } // import ./lib.nix;

    in
    scopedImport overrides ./imported.nix
  ))
]
