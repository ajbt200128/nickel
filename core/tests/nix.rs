use nickel_lang_core::term::Term;
use nickel_lang_utils::test_program::program_from_expr;
use test_generator::test_resources;

fn run(path: &str) {
    // remove the leading "core"
    let path = path.split('/').skip(1).collect::<Vec<_>>().join("/");
    let mut program = program_from_expr(format!(
        "(import \"{path}\") |> std.array.all std.function.id"
    ));
    program.add_import_paths(vec![env!("CARGO_MANIFEST_DIR")].into_iter());
    program
        .eval()
        .map(Term::from)
        .map(|term| {
            println!("{}: {:?}", path, term);
            assert_eq!(term, Term::Bool(true), "error in test {path}");
        })
        .unwrap();
}

#[test_resources("core/tests/nix/quick/*.nix")]
fn test_quick(resource: &str) {
    run(resource);
}

#[test_resources("core/tests/nix/basic/eval-okay-*.nix")]
fn test_basic(resource: &str) {
    run(resource);
}

//eval-okay-callable-attrs.nix: Nix functors are weird. We probably can
// replicate them but they're non trivial, and rarely used in NixOS/nixpkgs
//eval-okay-curpos.nix: Needs __curPos. Non trivial to get the column. Maybe we
// only provide __curPos.file, since that's all that's used in NixOS/nixpgs
//eval-okay-delayed-with.nix:  Overflows
//eval-okay-ind-string.nix: Nickel escapes some things differently than Nix,
// like dollar signs and tabs. Let's just ignore this test for now until we have
// a concrete example where this matters.
//eval-okay-null-dynamic-attrs.nix: TODO
//eval-okay-path-string-interpolation.nix: TODO
//eval-okay-patterns.nix: TODO
//eval-okay-remove.nix:  TODO
//eval-okay-scope-1.nix: TODO
//eval-okay-scope-3.nix: TODO
//eval-okay-scope-4.nix:  TODO
//eval-okay-scope-6.nix:  TODO
//eval-okay-scope-7.nix: TODO
//eval-okay-string.nix: TODO
//eval-okay-symlink-resolution.nix: TODO

// TODO: Implement nix builtins!
//#[test_resources("core/tests/nix/builtins/eval-okay-*.nix")]
//fn test_builtins(resource: &str) {
//    run(resource);
//}
