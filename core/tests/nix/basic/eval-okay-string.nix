# Note this test no longer passes in nix, and has been modified to pass in nickel
# This is because nickel doesn't do any path normalization, and nix does
[
  (("foobar/a/b/c/d/foo/bar/../xyzzy/./foo.txt/../foo/x/yescape: \"quote\" \n \\end\nof\nlinefoobarblaatfoo$bar$\"$\"$") == ("foo" + "bar"
    + toString (/a/b + /c/d)
    + toString (/foo/bar + "/../xyzzy/." + "/foo.txt")
    + ("/../foo" + toString /x/y)
    + "escape: \"quote\" \n \\"
    + "end
of
line"
    + "foo${if true then "b${"a" + "r"}" else "xyzzy"}blaat"
    + "foo$bar"
    + "$\"$\""
    + "$"))
]
