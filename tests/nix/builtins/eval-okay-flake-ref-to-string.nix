[
  (("github:NixOS/nixpkgs/23.05?dir=lib") == (builtins.flakeRefToString {
    type = "github";
    owner = "NixOS";
    repo = "nixpkgs";
    ref = "23.05";
    dir = "lib";
  }))
]
