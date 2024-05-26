#Nix compatibility layer.
#This library is used by program transpiled from Nix code to Nickel. This
#library shouldn't usually be used directly by Nickel program. The API
#isn't stable and no backward-compatibility guarantees exist at this
#point
 {
    # Addition in Nix is overloaded to work both as number addition and string
    # concatenation. There is no such operator in Nickel. This function
    # implement the equivalent of the Nix primitive operator by dynamically
    # dispatching between addition and concatenation, based on the runtime type
    # of its arguments.
    add = fun a b =>
      if %typeof% a == 'String && %typeof% b == 'String
      then
        a ++ b
      else
        a + b,

    # The update operator of Nix `//`. It's a "general form" of the
    # `record.update` of Nickel.
    #
    # TODO: May be interesting to be adapted and integrated to the actual Nickel
    # stdlib.
    update_all = fun r1 r2 =>
      r2
      |> std.record.fields
      |> std.array.fold_left (fun acc key => std.record.update key r2."%{key}" acc) r1,

    has_field_path = fun fields record =>
       # Because it's only used by generated code, this length will never be
       # initially 0. So if it's 0, it mean the end of the path.
       let head = std.array.first fields in
       std.array.length fields == 0 ||
       (
          std.is_record record && std.record.has_field head record &&
          has_field_path (std.array.drop_first fields) record."%{head}"
       ),

    with =
      let AssertFound = fun label value =>
        if value.found then
          value.value
        else
          %blame% label
      in

      fun envs field => (
        std.array.fold_right (fun current acc =>
          if !acc.found && std.record.has_field field current
          then { value = current."%{field}", found = true}
          else acc
        )   {value = null, found = false} envs
      ) | AssertFound,

    # The `assert` function of Nix. It's a simple wrapper around the
    # `std.test.Assert` function of Nickel since the `assert` function of Nix is
    # not a contract or anything similar but a simple function.
    assert = fun condition body =>
      let _ = condition | std.test.Assert in
      body,

    base_name_of = fun path =>
      # If the path is empty, we return an empty string since that's what Nix
      # does, and std.array.last fails on an empty array.
      if std.string.length path == 0 then
        ""
      else
        let parts = std.string.split "/" path in
        std.array.last parts,
  }
