[
  (("This is an indented multi-line string\nliteral.  An amount of whitespace at\nthe start of each line matching the minimum\nindentation of all lines in the string\nliteral together will be removed.  Thus,\nin this case four spaces will be\nstripped from each line, even though\n  THIS LINE is indented six spaces.\n\nAlso, empty lines don't count in the\ndetermination of the indentation level (the\nprevious empty line has indentation 0, but\nit doesn't matter).\nIf the string starts with whitespace\n  followed by a newline, it's stripped, but\n  that's not the case here. Two spaces are\n  stripped because of the \"  \" at the start. \nThis line is indented\na bit further.\nAnti-quotations, like so, are\nalso allowed.\n  The \\ is not special here.\n' can be followed by any character except another ', e.g. 'x'.\nLikewise for $, e.g. $$ or $varName.\nBut ' followed by ' is special, as is $ followed by {.\nIf you want them, use anti-quotations: '', \${.\n   Tabs are not interpreted as whitespace (since we can't guess\n   what tab settings are intended), so don't use them.\n\tThis line starts with a space and a tab, so only one\n   space will be stripped from each line.\nAlso note that if the last line (just before the closing ' ')\nconsists only of whitespace, it's ignored.  But here there is\nsome non-whitespace stuff, so the line isn't removed. \nThis shows a hacky way to preserve an empty line after the start.\nBut there's no reason to do so: you could just repeat the empty\nline.\n  Similarly you can force an indentation level,\n  in this case to 2 spaces.  This works because the anti-quote\n  is significant (not whitespace).\nstart on network-interfaces\n\nstart script\n\n  rm -f /var/run/opengl-driver\n  ln -sf 123 /var/run/opengl-driver\n\n  rm -f /var/log/slim.log\n   \nend script\n\nenv SLIM_CFGFILE=abc\nenv SLIM_THEMESDIR=def\nenv FONTCONFIG_FILE=/etc/fonts/fonts.conf  \t\t\t\t# !!! cleanup\nenv XKB_BINDIR=foo/bin         \t\t\t\t# Needed for the Xkb extension.\nenv LD_LIBRARY_PATH=libX11/lib:libXext/lib:/usr/lib/          # related to xorg-sys-opengl - needed to load libglx for (AI)GLX support (for compiz)\n\nenv XORG_DRI_DRIVER_PATH=nvidiaDrivers/X11R6/lib/modules/drivers/ \n\nexec slim/bin/slim\nEscaping of ' followed by ': ''\nEscaping of $ followed by {: \${\nAnd finally to interpret \\n etc. as in a string: \n, \r, \t.\nfoo\n'bla'\nbar\ncut -d $'\\t' -f 1\nending dollar $$\n") == (
    let

      s1 = ''
        This is an indented multi-line string
        literal.  An amount of whitespace at
        the start of each line matching the minimum
        indentation of all lines in the string
        literal together will be removed.  Thus,
        in this case four spaces will be
        stripped from each line, even though
          THIS LINE is indented six spaces.

        Also, empty lines don't count in the
        determination of the indentation level (the
        previous empty line has indentation 0, but
        it doesn't matter).
      '';

      s2 = ''  If the string starts with whitespace
    followed by a newline, it's stripped, but
    that's not the case here. Two spaces are
    stripped because of the "  " at the start. 
  '';

      s3 = ''
        This line is indented
        a bit further.
      ''; # indentation of last line doesn't count if it's empty

      s4 = ''
        Anti-quotations, like ${if true then "so" else "not so"}, are
        also allowed.
      '';

      s5 = ''
          The \ is not special here.
        ' can be followed by any character except another ', e.g. 'x'.
        Likewise for $, e.g. $$ or $varName.
        But ' followed by ' is special, as is $ followed by {.
        If you want them, use anti-quotations: ${"''"}, ${"\${"}.
      '';

      s6 = ''  
    Tabs are not interpreted as whitespace (since we can't guess
    what tab settings are intended), so don't use them.
 	This line starts with a space and a tab, so only one
    space will be stripped from each line.
  '';

      s7 = ''
        Also note that if the last line (just before the closing ' ')
        consists only of whitespace, it's ignored.  But here there is
        some non-whitespace stuff, so the line isn't removed. '';

      s8 = ''    ${""}
    This shows a hacky way to preserve an empty line after the start.
    But there's no reason to do so: you could just repeat the empty
    line.
  '';

      s9 = ''
        ${""}  Similarly you can force an indentation level,
          in this case to 2 spaces.  This works because the anti-quote
          is significant (not whitespace).
      '';

      s10 = ''
  '';

      s11 = '''';

      s12 = ''   '';

      s13 = ''
        start on network-interfaces

        start script
    
          rm -f /var/run/opengl-driver
          ${if true
            then "ln -sf 123 /var/run/opengl-driver"
            else if true
            then "ln -sf 456 /var/run/opengl-driver"
            else ""
          }

          rm -f /var/log/slim.log
       
        end script

        env SLIM_CFGFILE=${"abc"}
        env SLIM_THEMESDIR=${"def"}
        env FONTCONFIG_FILE=/etc/fonts/fonts.conf  				# !!! cleanup
        env XKB_BINDIR=${"foo"}/bin         				# Needed for the Xkb extension.
        env LD_LIBRARY_PATH=${"libX11"}/lib:${"libXext"}/lib:/usr/lib/          # related to xorg-sys-opengl - needed to load libglx for (AI)GLX support (for compiz)

        ${if true
          then "env XORG_DRI_DRIVER_PATH=${"nvidiaDrivers"}/X11R6/lib/modules/drivers/"
        else if true
          then "env XORG_DRI_DRIVER_PATH=${"mesa"}/lib/modules/dri"
          else ""
        } 

        exec ${"slim"}/bin/slim
      '';

      s14 = ''
        Escaping of ' followed by ': '''
        Escaping of $ followed by {: ''${
        And finally to interpret \n etc. as in a string: ''\n, ''\r, ''\t.
      '';

      # Regression test: string interpolation in '${x}' should work, but didn't.
      s15 = let x = "bla"; in ''
        foo
        '${x}'
        bar
      '';

      # Regression test: accept $'.
      s16 = ''
        cut -d $'\t' -f 1
      '';

      # Accept dollars at end of strings 
      s17 = ''ending dollar $'' + ''$'' + "\n";

    in
    s1 + s2 + s3 + s4 + s5 + s6 + s7 + s8 + s9 + s10 + s11 + s12 + s13 + s14 + s15 + s16 + s17
  ))
]
