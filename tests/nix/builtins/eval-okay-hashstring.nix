[
  (([ "d41d8cd98f00b204e9800998ecf8427e" "6c69ee7f211c640419d5366cc076ae46" "bb3438fbabd460ea6dbd27d153e2233b" "da39a3ee5e6b4b0d3255bfef95601890afd80709" "cd54e8568c1b37cf1e5badb0779bcbf382212189" "6d12e10b1d331dad210e47fd25d4f260802b7e77" "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855" "900a4469df00ccbfd0c145c6d1e4b7953dd0afafadd7534e3a4019e8d38fc663" "ad0387b3bd8652f730ca46d25f9c170af0fd589f42e7f23f5a9e6412d97d7e56" "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e" "9d0886f8c6b389398a16257bc79780fab9831c7fc11c8ab07fa732cb7b348feade382f92617c9c5305fefba0af02ab5fd39a587d330997ff5bd0db19f7666653" "21644b72aa259e5a588cd3afbafb1d4310f4889680f6c83b9d531596a5a284f34dbebff409d23bcc86aee6bad10c891606f075c6f4755cb536da27db5693f3a7" ]) == (
    let
      strings = [ "" "text 1" "text 2" ];
    in
    builtins.concatLists (map (hash: map (builtins.hashString hash) strings) [ "md5" "sha1" "sha256" "sha512" ])
  ))
]