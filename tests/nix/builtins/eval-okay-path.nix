[
  (([ "/nix/store/ya937r4ydw0l6kayq8jkyqaips9c75jm-output" "/nix/store/m7y372g6jb0g4hh1dzmj847rd356fhnz-output" ]) == ([
    (builtins.path
      {
        path = ./.;
        filter = path: _: baseNameOf path == "data";
        recursive = true;
        sha256 = "1yhm3gwvg5a41yylymgblsclk95fs6jy72w0wv925mmidlhcq4sw";
        name = "output";
      })
    (builtins.path
      {
        path = ./data;
        recursive = false;
        sha256 = "0k4lwj58f2w5yh92ilrwy9917pycipbrdrr13vbb3yd02j09vfxm";
        name = "output";
      })
  ]))
]
