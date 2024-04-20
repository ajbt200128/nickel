[
  (("<?xml version='1.0' encoding='utf-8'?>\n<expr>\n  <attrs>\n    <attr name=\"a\">\n      <string value=\"s\" />\n    </attr>\n  </attrs>\n</expr>\n") == (# Make sure the expected XML output is produced; in particular, make sure it
    # doesn't contain source location information.
    builtins.toXML { a = "s"; }))
]
