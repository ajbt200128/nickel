use crate::cache::Cache;
use crate::conversion::State;
pub use crate::conversion::ToNickel;
use crate::identifier::LocIdent;
use crate::label::{Label, MergeLabel};
use crate::mk_app;
use crate::parser::utils::{build_record, mk_span, AttachTerm, FieldDef, FieldPathElem};
use crate::position::{RawSpan, TermPos};
use crate::term::make::{self, if_then_else};
use crate::term::record::{FieldMetadata, RecordAttrs};
use crate::term::{record::Field, RichTerm, Term};
use crate::term::{record::RecordData, BinaryOp, UnaryOp};
use crate::term::{LabeledType, MergePriority, TypeAnnotation};
use crate::typ::{Type, TypeF};
use codespan::FileId;
use indexmap::IndexMap;
use rnix::ast::{
    AstNode, Attr as NixAttr, AttrpathValue, BinOp as NixBinOp, HasEntry, Ident as NixIdent,
    InterpolPart, Str as NixStr, UnaryOp as NixUniOp,
};
use rowan::ast::AstChildren;
use std::collections::HashMap;
use std::rc::Rc;

pub type NixParseError = rnix::parser::ParseError;
fn path_elem_from_nix(attr: NixAttr, state: &State) -> FieldPathElem {
    match attr {
        NixAttr::Ident(id) => FieldPathElem::Ident(id_from_nix(id, state)),
        NixAttr::Str(s) => FieldPathElem::Expr(s.translate(state)),
        NixAttr::Dynamic(d) => FieldPathElem::Expr(d.expr().unwrap().translate(state)),
    }
}

fn path_rts_from_nix<T>(n: rnix::ast::Attrpath, state: &State) -> T
where
    T: FromIterator<RichTerm>,
{
    n.attrs().map(|a| a.translate(state)).collect()
}

fn span_from_nix(node: &dyn AstNode, state: &State) -> RawSpan {
    let pos = node.syntax().text_range();
    mk_span(state.file_id, pos.start().into(), pos.end().into())
}

fn pos_from_nix(node: &dyn AstNode, state: &State) -> TermPos {
    TermPos::Original(span_from_nix(node, state))
}

fn id_from_nix(id: NixIdent, state: &State) -> LocIdent {
    let pos = pos_from_nix(&id, state);
    LocIdent::new_with_pos(id.to_string(), pos)
}

fn extend_env_with_attrset(state: &mut State, attrpath_values: AstChildren<AttrpathValue>) {
    state.env.extend(attrpath_values.map(|kv| {
        // TODO: does not work if the let contains Dynamic or Str
        // TODO: nix supports attrpaths that are nested i.e. a.b.c, we should
        // add the proper path to the env if it's the case.
        kv.attrpath().unwrap().attrs().next().unwrap().to_string()
    }));
}

fn str_chunks_from_interpol<T: ToString>(interpols: Vec<InterpolPart<T>>, state: &State) -> Term {
    Term::StrChunks(
        interpols
            .into_iter()
            .enumerate()
            .map(|(i, c)| match c {
                InterpolPart::Literal(s) => crate::term::StrChunk::Literal(s.to_string()),
                InterpolPart::Interpolation(interp) => {
                    crate::term::StrChunk::Expr(interp.expr().unwrap().translate(state), i)
                }
            })
            // Parts come in reverse order
            .rev()
            .collect(),
    )
}

fn record_field_from_attrpath_value(kv: AttrpathValue, state: &State) -> (FieldPathElem, Field) {
    let path = kv
        .attrpath()
        .unwrap()
        .attrs()
        .map(|e| path_elem_from_nix(e, state))
        .collect();
    let value = kv.value().unwrap().translate(state);
    let pos = pos_from_nix(&kv, state);
    (FieldDef {
        path,
        field: Field::from(value),
        pos,
    })
    .elaborate()
}

impl ToNickel for Vec<AttrpathValue> {
    fn translate(self, state: &State) -> RichTerm {
        let fields: Vec<_> = self
            .into_iter()
            .map(|kv| record_field_from_attrpath_value(kv, state))
            .collect();
        build_record(fields, Default::default()).into()
    }
}

impl ToNickel for NixAttr {
    fn translate(self, state: &State) -> RichTerm {
        match self {
            NixAttr::Ident(id) => Term::Str(id.to_string().into()).into(),
            NixAttr::Str(s) => s.translate(state),
            NixAttr::Dynamic(d) => d.expr().unwrap().translate(state),
        }
    }
}

impl ToNickel for NixStr {
    fn translate(self, state: &State) -> RichTerm {
        let pos = pos_from_nix(&self, state);
        let chunks = str_chunks_from_interpol(self.normalized_parts(), state);

        RichTerm::new(chunks, pos)
    }
}

impl ToNickel for NixUniOp {
    fn translate(self, state: &State) -> RichTerm {
        use rnix::ast::UnaryOpKind::*;
        let value = self.expr().unwrap().translate(state);
        match self.operator().unwrap() {
            Negate => make::op2(BinaryOp::Sub(), Term::Num((0. as i64).into()), value),
            Invert => make::op1(UnaryOp::BoolNot(), value),
        }
    }
}

impl ToNickel for NixBinOp {
    fn translate(self, state: &State) -> RichTerm {
        use rnix::ast::BinOpKind::*;
        let lhs = self.lhs().unwrap().translate(state);
        let rhs = self.rhs().unwrap().translate(state);
        match self.operator().unwrap() {
            Concat => make::op2(BinaryOp::ArrayConcat(), lhs, rhs),
            Update => mk_app!(crate::stdlib::compat::update(), lhs, rhs),

            // Use a compatibility function to be able to merge strings with the same operator used
            // for addition.
            Add => mk_app!(crate::stdlib::compat::add(), lhs, rhs),
            Sub => make::op2(BinaryOp::Sub(), lhs, rhs),
            Mul => make::op2(BinaryOp::Mult(), lhs, rhs),
            Div => make::op2(BinaryOp::Div(), lhs, rhs),

            Equal => make::op2(BinaryOp::Eq(), lhs, rhs),
            Less => make::op2(BinaryOp::LessThan(), lhs, rhs),
            More => make::op2(BinaryOp::GreaterThan(), lhs, rhs),
            LessOrEq => make::op2(BinaryOp::LessOrEq(), lhs, rhs),
            MoreOrEq => make::op2(BinaryOp::GreaterOrEq(), lhs, rhs),
            NotEqual => make::op1(UnaryOp::BoolNot(), make::op2(BinaryOp::Eq(), lhs, rhs)),

            // the Nix `->` operator.
            // if the lhs is true, then it return the boolean value of rhs. If lhs is false, the
            // implication is alwais true.
            Implication => if_then_else(lhs, rhs, Term::Bool(true)),

            // In Nickel as oposit to Nix, the `&&` and `||` operators are unary operators.
            And => mk_app!(Term::Op1(UnaryOp::BoolAnd(), lhs), rhs),
            Or => mk_app!(Term::Op1(UnaryOp::BoolOr(), lhs), rhs),
        }
    }
}

impl ToNickel for rnix::ast::Expr {
    fn translate(self, state: &State) -> RichTerm {
        use rnix::ast::Expr;
        let pos = pos_from_nix(&self, state);

        #[cfg(debug_assertions)]
        eprintln!("{self:?}: {self}");
        match self {
            // This is a parse error of the nix code.
            // it's translated to a Nickel internal error specific for nix code (`NixParseError`)
            // May not be the better way to do, but this version of the code does not realy have
            // error management for the nix side.
            Expr::Error(_) => {
                Term::ParseError(crate::error::ParseError::NixParseError(state.file_id)).into()
                // TODO: Improve error management
            }
            // The Root of a file. generaly, this field is not matched because the common way to
            // translate is as we do in `parse` function below. Like this, we pass a actual `Expr`
            // to this function and not the `Root` wrapper.
            // Anyway we prefer to manage it, in case the caller pass a `Expr` casted from
            // `rowan::AstNode`.
            Expr::Root(n) => n.expr().unwrap().translate(state),
            Expr::Paren(n) => n.expr().unwrap().translate(state),

            // nix's assert always returns a separate body when the assertion
            // succeeds, not a boolean. Nickel's assertion is simply a contract
            // so we want to emulate that here. Let's just discard the boolean
            // result of the statement after asserting and return the 2nd expression
            Expr::Assert(n) => {
                let condition = n.condition().unwrap().translate(state);
                let body = n.body().unwrap().translate(state);
                mk_app!(crate::stdlib::compat::assert(), condition, body)
            }

            // Some specificity around Nix literals or better said, on how `rnix` parse the
            // literals:
            // - It differenciate floats and integers. We then convertboth to floats.
            // - For some reason, `Uri`s are concidered literals, but `Str` and `Path` are not.
            Expr::Literal(n) => match n.kind() {
                rnix::ast::LiteralKind::Float(v) => Term::Num((v.value().unwrap() as i64).into()),
                rnix::ast::LiteralKind::Integer(v) => Term::Num((v.value().unwrap() as i64).into()),
                // TODO: How to manage Uris in nickel?
                // What should be the nickel internal representation?
                // String could be ok, but what if we give it back to a Nix expr?
                // Apologise, not sure of the output of `Uri::to_string`
                rnix::ast::LiteralKind::Uri(v) => Term::Str(v.to_string().into()),
            }
            .into(),
            // That's what we call a multiline string in Nickel. Nix don't have the concept of
            // string literal (e.g.: `Term::Str` of Nickel)
            Expr::Str(n) => n.translate(state),
            Expr::List(n) => Term::Array(
                n.items().map(|elm| elm.translate(state)).collect(),
                Default::default(),
            )
            .into(),
            Expr::AttrSet(n) => {
                let mut state = state.clone();
                // check if the attrset is recursive
                let (attrpath_values, skipped_attrpath_values) = match n.rec_token() {
                    // check if the attrset is recursive and fill the environment with the fields if so
                    Some(_) => {
                        extend_env_with_attrset(&mut state, n.attrpath_values());
                        (n.attrpath_values().collect(), vec![])
                    }
                    // If it isn't, then we partition out all the record fields
                    // who will shadow existing variables. Then we can create a
                    // separate record for those fields.
                    //
                    // For example:
                    // nix
                    //   ((x: { x = 1; y = x; }) 2) == { x = 1; y = 2; }
                    // translates to
                    // nickel
                    //   (fun x => { x = 1, y = x }) 3 == { x = 1, y = 1, }
                    // but by splitting it out we get
                    // nickel
                    //   (fun x => { y = x } & { x = 1 }) 2 == { x = 1, y = 2, }
                    None => {
                        // `partition` doesn't work here for some reason
                        let vals: Vec<_> = n
                            .attrpath_values()
                            .filter(|kv| {
                                let id = kv.attrpath().unwrap().attrs().next().unwrap().to_string();
                                !state.env.contains(&id)
                            })
                            .collect();
                        let skipped_vals: Vec<_> = n
                            .attrpath_values()
                            .filter(|kv| {
                                let id = kv.attrpath().unwrap().attrs().next().unwrap().to_string();
                                state.env.contains(&id)
                            })
                            .collect();
                        (vals, skipped_vals)
                    }
                };
                let initial_record = attrpath_values.translate(&state);
                // When nix attr sets are not recursive, they always set values
                // to what's in scope.
                if skipped_attrpath_values.is_empty() {
                    initial_record
                } else {
                    let skipped_record = skipped_attrpath_values.translate(&state);
                    let span = span_from_nix(&n, &state);
                    make::op2(
                        BinaryOp::Merge(MergeLabel {
                            span,
                            kind: crate::label::MergeKind::Standard,
                        }),
                        initial_record,
                        skipped_record,
                    )
                }
            }

            // In nix it's allowed to define vars named `true`, `false` or `null`.
            // But we prefer to not support it. If we try to redefine one of these builtins, nickel
            // will panic (see below in the `LetIn` arm).
            Expr::Ident(id) => match id.to_string().as_str() {
                "true" => Term::Bool(true),
                "false" => Term::Bool(false),
                "null" => Term::Null,
                "baseNameOf" => crate::stdlib::compat::base_name_of().into(),
                "toString" => crate::stdlib::compat::to_string().into(),
                "removeAttrs" => crate::stdlib::compat::remove_attrs().into(),
                id_str => {
                    // Compatibility with the Nix `with` construct. It look if the identifier has
                    // been staticaly defined and if not, it look for it in the `with` broughts
                    // identifiers.
                    if state.env.contains(id_str) || state.with.is_empty() {
                        Term::Var(id_from_nix(id, state))
                    } else {
                        Term::App(
                            crate::stdlib::compat::with(state.with.clone().into_iter().collect()),
                            Term::Str(id.to_string().into()).into(),
                        )
                    }
                }
            }
            .into(),
            Expr::LegacyLet(_) => panic!("Legacy let form is not supported"), // Probably useless to support it in a short term.
            // `let ... in` blocks are recursive in Nix and not in Nickel. To emulate this, we use
            // a `let <pattern> = <recrecord> in`. The record provide recursivity then the values
            // are destructured by the pattern.
            Expr::LetIn(n) => {
                use crate::term::pattern::*;
                let mut patterns_vec = Vec::new();
                let mut fields = HashMap::new();
                let mut state = state.clone();
                extend_env_with_attrset(&mut state, n.attrpath_values());
                for kv in n.attrpath_values() {
                    // In `let` blocks, the key is supposed to be a single ident so `Path` exactly one
                    // element.
                    let id = kv.attrpath().unwrap().attrs().next().unwrap();
                    // Check we don't try to redefine builtin values. Even if it's possible in Nix,
                    // we don't suport it.
                    let id: LocIdent = match id.to_string().as_str() {
                        "true" | "false" | "null" => panic!(
                            "`let {id}` is forbidden. Can not redefine `true`, `false` or `null`"
                        ),
                        s => {
                            let pos = pos_from_nix(&id, &state);
                            // give a position to the identifier.
                            LocIdent::new_with_pos(s, pos)
                        }
                    };
                    let rt = kv.value().unwrap().translate(&state);
                    let annotation = TypeAnnotation {
                        typ: None,
                        contracts: vec![],
                    };

                    let data = PatternData::Any(id);
                    let pattern = Pattern {
                        data,
                        alias: None,
                        pos: id.pos,
                    };
                    let field_pattern = FieldPattern {
                        matched_id: id,
                        annotation,
                        default: None,
                        pattern,
                        pos: id.pos,
                    };
                    patterns_vec.push(field_pattern);
                    fields.insert(id, rt);
                }
                let record_pattern = RecordPattern {
                    patterns: patterns_vec,
                    tail: TailPattern::Empty,
                    pos,
                };
                let pattern = Pattern {
                    data: PatternData::Record(record_pattern),
                    alias: None,
                    pos,
                };

                make::let_pat(
                    pattern,
                    Term::RecRecord(RecordData::with_field_values(fields), Vec::new(), None),
                    n.body().unwrap().translate(&state),
                )
            }
            Expr::With(n) => {
                let mut state = state.clone();
                // we push in a vec the term passed to the with (e.g.: `with t; ...` we push the
                // term `t`) we push a term because it does not to have a variable, it can be any
                // expretion evaluated to a record.
                state.with.push(n.namespace().unwrap().translate(&state));
                // In the Nickel AST, a with don't realy exist. It's translated to its body. That's
                // only when we will parse a variable access that we will take care of the `with`s.
                // See the `Expr::Identifier` of the current `match`.
                n.body().unwrap().translate(&state)
            }

            // a lambda or a function definition.
            Expr::Lambda(n) => {
                // no matter what we're going to add the param to the environment.
                let mut state = state.clone();
                match n.param().unwrap() {
                    // the simple case in which the param of the lambda is an identifier as in
                    // `f = x: ...` x is an identifier.
                    rnix::ast::Param::IdentParam(idp) => {
                        let idp_ident = idp.ident().unwrap();
                        state.env.insert(idp_ident.to_string());
                        Term::Fun(
                            id_from_nix(idp_ident, &state),
                            n.body().unwrap().translate(&state),
                        )
                    }
                    // the param is a pattern as we generaly see in NixOS modules (`{pkgs, lib,
                    // ...}:`
                    rnix::ast::Param::Pattern(pat) => {
                        // TODO: Does not support if args are empty, e.g. nix`{}: 1`
                        use crate::term::pattern::*;
                        // Pattern alias i.e. args@{x,y,z}:
                        let alias = match pat.pat_bind() {
                            Some(bind) => Some(id_from_nix(bind.ident().unwrap(), &state)),
                            None => None,
                        };

                        // Pattern allows additional entries i.e. {x,y,z,...}:
                        let open = pat.ellipsis_token().is_some();

                        // So nix allows recursive pattern matching in lambda
                        // patterns:
                        //   f = {x ? y, y ? z, z ? x}
                        // But nickel does not, and we've decided that nickel
                        // doesn't need that functionality for now as it would
                        // break some things.
                        // So instead what we do here is when we encounter a
                        // lambda we set the defaults via a contract on the
                        // lambda instead of the normal approach, as contracts
                        // allow this recursive default property we want
                        // The drawback is that if a user alias's the pattern in
                        // the nix code, then uses that alias IN the pattern
                        // defaults, we will translate it, but it the alias in
                        // the default will be undefined
                        let mut contract_fields = IndexMap::new();

                        let pos = pos_from_nix(&pat, &state);

                        let patterns = pat
                            .pat_entries()
                            .map(|e| {
                                let e_ident = e.ident().unwrap();
                                state.env.insert(e_ident.to_string());
                                let id = id_from_nix(e_ident, &state);

                                // Create a field whose default is that of the
                                // nix default
                                let metadata = FieldMetadata {
                                    doc: None,
                                    annotation: TypeAnnotation {
                                        typ: None,
                                        contracts: vec![],
                                    },
                                    opt: false,
                                    not_exported: false,
                                    priority: MergePriority::Bottom,
                                };
                                let value = e.default().map(|d| d.translate(&state));
                                let field = Field {
                                    value,
                                    metadata,
                                    pending_contracts: vec![],
                                };
                                contract_fields.insert(id, field.into());

                                let annotation = TypeAnnotation {
                                    typ: None,
                                    contracts: vec![],
                                };
                                let data = PatternData::Any(id);
                                let pattern = Pattern {
                                    data,
                                    alias: None,
                                    pos: id.pos,
                                };
                                FieldPattern {
                                    matched_id: id,
                                    annotation,
                                    // Handled by contract_fields
                                    default: None,
                                    pattern,
                                    pos: id.pos,
                                }
                            })
                            .collect();
                        // This record is what the contract that provides the
                        // defaults will be
                        let contract_record = Term::RecRecord(
                            RecordData::new(
                                contract_fields,
                                RecordAttrs {
                                    open,
                                    closurized: false,
                                },
                                Default::default(),
                            ),
                            vec![],
                            None,
                        );
                        // Construct the type of the lambda:
                        // {param1 | default = default1, [...]} -> Dyn
                        let typ_f = TypeF::Arrow(
                            Box::new(Type::from(TypeF::Flat(contract_record.into()))),
                            Box::new(Type::from(TypeF::Dyn)),
                        );
                        let typ = Type::from(typ_f);
                        // Create annotation, so | {param1 | default = default1, [...]} -> Dyn
                        let annotation = TypeAnnotation {
                            typ: None,
                            contracts: vec![LabeledType {
                                typ: typ.clone(),
                                label: Label {
                                    typ: Rc::new(typ),
                                    span: span_from_nix(&n, &state),
                                    ..Default::default()
                                },
                            }],
                        };
                        // Now let's create the actual lambda

                        // Create the pattern
                        let record_pattern = RecordPattern {
                            patterns,
                            tail: if open {
                                TailPattern::Open
                            } else {
                                TailPattern::Empty
                            },
                            pos,
                        };
                        let pattern = Pattern {
                            data: PatternData::Record(record_pattern),
                            alias,
                            pos,
                        };

                        // Create the lambda
                        let fun = Term::FunPattern(pattern, n.body().unwrap().translate(&state));
                        // attach the annotation so now we have
                        //   (fun {param1, param2, [...]} => body)
                        //     | {param1 | default = default1, [...]}
                        annotation.attach_term(fun.into()).into()
                    }
                }
            }
            .into(),

            // function application.
            Expr::Apply(n) => Term::App(
                n.lambda().unwrap().translate(state),
                n.argument().unwrap().translate(state),
            )
            .into(),
            Expr::IfElse(n) => if_then_else(
                n.condition().unwrap().translate(state),
                n.body().unwrap().translate(state),
                n.else_body().unwrap().translate(state),
            ),
            Expr::BinOp(n) => n.translate(state),
            Expr::UnaryOp(n) => n.translate(state),

            // static or dynamic records field access.
            Expr::Select(n) => {
                let select = n
                    .attrpath()
                    .unwrap()
                    .attrs()
                    // a nested access is an iterator on attrs from left to right.
                    .fold(
                        n.expr().unwrap().translate(state), // the fold is initialized with the
                        // record accessed.
                        |acc, i| {
                            match i {
                                rnix::ast::Attr::Ident(id) => {
                                    Term::Op1(UnaryOp::StaticAccess(id_from_nix(id, state)), acc)
                                }
                                rnix::ast::Attr::Dynamic(d) => Term::Op2(
                                    BinaryOp::DynAccess(),
                                    d.expr().unwrap().translate(state),
                                    acc,
                                ),
                                rnix::ast::Attr::Str(s) => {
                                    Term::Op2(BinaryOp::DynAccess(), s.translate(state), acc)
                                }
                            }
                            .into()
                        },
                    );
                // if the selection contains a `... or <default>` suffix
                if let Some(def) = n.default_expr() {
                    let path = path_rts_from_nix(n.attrpath().unwrap(), state);
                    let path = Term::Array(path, Default::default());
                    // we transform it to something like the following pseudo code:
                    //
                    // ```
                    // if has_field_path <path> <record>
                    // then <record>.<path>
                    // else <default>
                    // ```
                    if_then_else(
                        mk_app!(
                            crate::stdlib::compat::has_field_path(),
                            path,
                            n.expr().unwrap().translate(state)
                        ),
                        select,
                        def.translate(state),
                    )
                } else {
                    select
                }
            }

            // The Nix `?` operator.
            Expr::HasAttr(n) => {
                let path = path_rts_from_nix(n.attrpath().unwrap(), state);
                let path = Term::Array(path, Default::default());
                mk_app!(
                    crate::stdlib::compat::has_field_path(),
                    path,
                    n.expr().unwrap().translate(state)
                )
            }
            Expr::Path(n) => {
                // lets just add the path as a string since nickel doesn't have a path syntax
                // and just uses strings
                let parts = n.parts().collect::<Vec<_>>();

                // join with "/" to have a string representation of the path.
                // TODO: Do we support windows paths? Probably not...
                //let path = parts.collect::<Vec<_>>().join("/");
                let chunks = str_chunks_from_interpol(parts, state);
                RichTerm::new(chunks, pos)
            }
        }
        // set the position in the AST to try to have some sort of debuging support.
        .with_pos(pos)
    }
}

/// the main entry of this module. It parse a Nix file pointed by `file_id` into a Nickel
/// AST/Richterm.
pub fn parse(cache: &Cache, file_id: FileId) -> Result<RichTerm, NixParseError> {
    let source = cache.files().source(file_id);
    let root = rnix::Root::parse(source).ok()?; // TODO: we could return a list of errors calling
                                                // `errors()` to improve error management.
    Ok(root.expr().unwrap().to_nickel(file_id))
}
