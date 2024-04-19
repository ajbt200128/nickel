#![cfg(feature = "nix")]
use nickel_lang::term::Term;
use nickel_lang_utilities::eval;
use test_generator::test_resources;

fn run(path: &str) {
    eval(format!(
        "import \"{}/{path}\" |> array.all function.id",
        env!("CARGO_MANIFEST_DIR"),
    ))
    .map(|term| {
        assert_eq!(term, Term::Bool(true), "error in test {path}");
    })
    .unwrap();
}

#[test_resources("tests/nix/quick/*.nix")]
fn test_quick(resource: &str) {
    run(resource);
}

#[test_resources("tests/nix/basic/eval-okay-*.nix")]
fn test_basic(resource: &str) {
    run(resource);
}

//eval-okay-autoargs.nix: TODO
//eval-okay-attrs.nix: Needs assert
//eval-okay-attrs2.nix:  Needs assert
//eval-okay-attrs3.nix: TODO
//eval-okay-attrs4.nix: TODO
//eval-okay-attrs5.nix: TODO
//eval-okay-attrs6.nix: TODO
//eval-okay-backslash-newline-1.nix: TODO
//eval-okay-backslash-newline-2.nix: TODO
//eval-okay-baseNameOf.nix:  Needs assert
//eval-okay-callable-attrs.nix:  Overflows
//eval-okay-curpos.nix:  TODO
//eval-okay-delayed-with.nix:  Overflows
//eval-okay-dynamic-attrs-2.nix: TODO
//eval-okay-dynamic-attrs-bare.nix: TODO
//eval-okay-dynamic-attrs.nix: TODO
//eval-okay-empty-args.nix: TODO
//eval-okay-flatten.nix: TODO
//eval-okay-functionargs.nix: TODO
//eval-okay-import.nix: TODO
//eval-okay-ind-string.nix: TODO
//eval-okay-list.nix: needs TODO
//eval-okay-listtoattrs.nix: TODO
//eval-okay-logic.nix:  needs assert
//eval-okay-map.nix: TODO
//eval-okay-merge-dynamic-attrs.nix: TODO
//eval-okay-nested-with.nix: TODO
//eval-okay-null-dynamic-attrs.nix: TODO
//eval-okay-overrides.nix: TODO
//eval-okay-path-string-interpolation.nix: TODO
//eval-okay-patterns.nix: TODO
//eval-okay-remove.nix:  TODO
//eval-okay-scope-1.nix: TODO
//eval-okay-scope-3.nix: TODO
//eval-okay-scope-4.nix:  TODO
//eval-okay-scope-6.nix:  TODO
//eval-okay-scope-7.nix: TODO
//eval-okay-string.nix: TODO
//eval-okay-strings-as-attrs-names.nix: TODO
//eval-okay-symlink-resolution.nix: TODO
//eval-okay-tail-call-1.nix: TODO
//eval-okay-xml.nix: TODO

#[test_resources("tests/nix/builtins/eval-okay-*.nix")]
fn test_builtins(resource: &str) {
    run(resource);
}