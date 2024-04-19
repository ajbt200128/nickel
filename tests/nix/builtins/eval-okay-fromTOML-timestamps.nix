[
  (({ "1234" = "value"; "127.0.0.1" = "value"; a = { b = { c = { }; }; }; arr1 = [ 1 2 3 ]; arr2 = [ "red" "yellow" "green" ]; arr3 = [ [ 1 2 ] [ 3 4 5 ] ]; arr4 = [ "all" "strings" "are the same" "type" ]; arr5 = [ [ 1 2 ] [ "a" "b" "c" ] ]; arr7 = [ 1 2 3 ]; arr8 = [ 1 2 ]; bare-key = "value"; bare_key = "value"; bin1 = 214; bool1 = true; bool2 = false; "character encoding" = "value"; d = { e = { f = { }; }; }; dog = { "tater.man" = { type = { name = "pug"; }; }; }; flt1 = 1; flt2 = 3.1415; flt3 = -0.01; flt4 = 5 e + 22; flt5 = 1 e + 06; flt6 = -0.02; flt7 = 6.626e-34; flt8 = 9.22462e+06; fruit = [{ name = "apple"; physical = { color = "red"; shape = "round"; }; variety = [{ name = "red delicious"; } { name = "granny smith"; }]; } { name = "banana"; variety = [{ name = "plantain"; }]; }]; g = { h = { i = { }; }; }; hex1 = 3735928559; hex2 = 3735928559; hex3 = 3735928559; int1 = 99; int2 = 42; int3 = 0; int4 = -17; int5 = 1000; int6 = 5349221; int7 = 12345; j = { "ʞ" = { l = { }; }; }; key = "value"; key2 = "value"; ld1 = { _type = "timestamp"; value = "1979-05-27"; }; ldt1 = { _type = "timestamp"; value = "1979-05-27T07:32:00"; }; ldt2 = { _type = "timestamp"; value = "1979-05-27T00:32:00.999999"; }; lt1 = { _type = "timestamp"; value = "07:32:00"; }; lt2 = { _type = "timestamp"; value = "00:32:00.999999"; }; name = "Orange"; oct1 = 342391; oct2 = 493; odt1 = { _type = "timestamp"; value = "1979-05-27T07:32:00Z"; }; odt2 = { _type = "timestamp"; value = "1979-05-27T00:32:00-07:00"; }; odt3 = { _type = "timestamp"; value = "1979-05-27T00:32:00.999999-07:00"; }; odt4 = { _type = "timestamp"; value = "1979-05-27T07:32:00Z"; }; physical = { color = "orange"; shape = "round"; }; products = [{ name = "Hammer"; sku = 738594937; } { } { color = "gray"; name = "Nail"; sku = 284758393; }]; "quoted \"value\"" = "value"; site = { "google.com" = true; }; str = "I'm a string. \"You can quote me\". Name\tJosé\nLocation\tSF."; table-1 = { key1 = "some string"; key2 = 123; }; table-2 = { key1 = "another string"; key2 = 456; }; x = { y = { z = { w = { animal = { type = { name = "pug"; }; }; name = { first = "Tom"; last = "Preston-Werner"; }; point = { x = 1; y = 2; }; }; }; }; }; "ʎǝʞ" = "value"; }) == (builtins.fromTOML ''
    key = "value"
    bare_key = "value"
    bare-key = "value"
    1234 = "value"

    "127.0.0.1" = "value"
    "character encoding" = "value"
    "ʎǝʞ" = "value"
    'key2' = "value"
    'quoted "value"' = "value"

    name = "Orange"

    physical.color = "orange"
    physical.shape = "round"
    site."google.com" = true

    # This is legal according to the spec, but cpptoml doesn't handle it.
    #a.b.c = 1
    #a.d = 2

    str = "I'm a string. \"You can quote me\". Name\tJos\u00E9\nLocation\tSF."

    int1 = +99
    int2 = 42
    int3 = 0
    int4 = -17
    int5 = 1_000
    int6 = 5_349_221
    int7 = 1_2_3_4_5

    hex1 = 0xDEADBEEF
    hex2 = 0xdeadbeef
    hex3 = 0xdead_beef

    oct1 = 0o01234567
    oct2 = 0o755

    bin1 = 0b11010110

    flt1 = +1.0
    flt2 = 3.1415
    flt3 = -0.01
    flt4 = 5e+22
    flt5 = 1e6
    flt6 = -2E-2
    flt7 = 6.626e-34
    flt8 = 9_224_617.445_991_228_313

    bool1 = true
    bool2 = false

    odt1 = 1979-05-27T07:32:00Z
    odt2 = 1979-05-27T00:32:00-07:00
    odt3 = 1979-05-27T00:32:00.999999-07:00
    odt4 = 1979-05-27 07:32:00Z
    ldt1 = 1979-05-27T07:32:00
    ldt2 = 1979-05-27T00:32:00.999999
    ld1 = 1979-05-27
    lt1 = 07:32:00
    lt2 = 00:32:00.999999

    arr1 = [ 1, 2, 3 ]
    arr2 = [ "red", "yellow", "green" ]
    arr3 = [ [ 1, 2 ], [3, 4, 5] ]
    arr4 = [ "all", 'strings', """are the same""", ''''type'''']
    arr5 = [ [ 1, 2 ], ["a", "b", "c"] ]

    arr7 = [
      1, 2, 3
    ]

    arr8 = [
      1,
      2, # this is ok
    ]

    [table-1]
    key1 = "some string"
    key2 = 123


    [table-2]
    key1 = "another string"
    key2 = 456

    [dog."tater.man"]
    type.name = "pug"

    [a.b.c]
    [ d.e.f ]
    [ g .  h  . i ]
    [ j . "ʞ" . 'l' ]
    [x.y.z.w]

    name = { first = "Tom", last = "Preston-Werner" }
    point = { x = 1, y = 2 }
    animal = { type.name = "pug" }

    [[products]]
    name = "Hammer"
    sku = 738594937

    [[products]]

    [[products]]
    name = "Nail"
    sku = 284758393
    color = "gray"

    [[fruit]]
      name = "apple"

      [fruit.physical]
        color = "red"
        shape = "round"

      [[fruit.variety]]
        name = "red delicious"

      [[fruit.variety]]
        name = "granny smith"

    [[fruit]]
      name = "banana"

      [[fruit.variety]]
        name = "plantain"
  ''))
]
