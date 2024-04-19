[
  (("<?xml version='1.0' encoding='utf-8'?>\n<expr>\n  <list>\n    <string value=\"ab\" />\n    <int value=\"10\" />\n    <attrs>\n      <attr name=\"x\">\n        <string value=\"x\" />\n      </attr>\n      <attr name=\"y\">\n        <string value=\"x\" />\n      </attr>\n    </attrs>\n  </list>\n</expr>\n") == (builtins.toXML [ ("a" + "b") 10 (rec { x = "x"; y = x; }) ]))
]
