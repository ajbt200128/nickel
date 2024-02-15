use crate::{
    error::TypecheckError,
    identifier::LocIdent,
    mk_uty_record_row,
    term::pattern::*,
    typ::{EnumRowsF, RecordRowsF, TypeF},
};

use super::{
    mk_uniftype, Context, State, UnifEnumRow, UnifEnumRows, UnifRecordRow, UnifRecordRows,
    UnifType, Unify, VarLevelsData,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum TypecheckMode {
    Walk,
    Enforce,
}

pub type TypeBindings = Vec<(LocIdent, UnifType)>;

pub(super) trait PatternTypes {
    /// The type produced by the pattern. Depending on the nature of the pattern, this type may
    /// vary: for example, a record pattern will record rows, while a general pattern will produce
    /// a general [super::UnifType]
    type PatType;

    /// Builds the type associated to the whole pattern, as well as the types associated to each
    /// binding introduced by this pattern. When matching a value against a pattern in a statically
    /// typed code, either by destructuring or by applying a match expression, the type of the
    /// value will be checked against the type generated by `pattern_type` and the bindings will be
    /// added to the type environment.
    ///
    /// The type of each "leaf" identifier will be assigned based on the `mode` argument. The
    /// current possibilities are for each leaf to have type `Dyn`, to use an explicit type
    /// annotation, or to be assigned a fresh unification variable.
    fn pattern_types(
        &self,
        state: &mut State,
        ctxt: &Context,
        mode: TypecheckMode,
    ) -> Result<(Self::PatType, TypeBindings), TypecheckError> {
        let mut bindings = Vec::new();
        let typ = self.pattern_types_inj(&mut bindings, state, ctxt, mode)?;
        Ok((typ, bindings))
    }

    /// Same as `pattern_types`, but inject the bindings in a working vector instead of returning
    /// them. Implementors should implement this method whose signature avoid creating and
    /// combining many short-lived vectors when walking recursively through a pattern.
    fn pattern_types_inj(
        &self,
        bindings: &mut Vec<(LocIdent, UnifType)>,
        state: &mut State,
        ctxt: &Context,
        mode: TypecheckMode,
    ) -> Result<Self::PatType, TypecheckError>;
}

/// Builds the type associated to a record pattern. When matching a value against a pattern in a
/// statically typed code, for example in a let destructuring or via a match expression, the type
/// of the value will be checked against the type generated by `build_pattern_type`.
///
/// The type of each "leaf" identifier will be assigned based on the `mode` argument. The current
/// possibilities are for each leaf to have type `Dyn`, to use an explicit type annotation, or to
/// be assigned a fresh unification variable.
impl PatternTypes for RecordPattern {
    type PatType = UnifRecordRows;

    fn pattern_types_inj(
        &self,
        bindings: &mut Vec<(LocIdent, UnifType)>,
        state: &mut State,
        ctxt: &Context,
        mode: TypecheckMode,
    ) -> Result<Self::PatType, TypecheckError> {
        let tail = if self.is_open() {
            match mode {
                // We use a dynamic tail here since we're in walk mode,
                // but if/when we remove dynamic record tails this could
                // likely be made an empty tail with no impact.
                TypecheckMode::Walk => mk_uty_record_row!(; RecordRowsF::TailDyn),
                TypecheckMode::Enforce => state.table.fresh_rrows_uvar(ctxt.var_level),
            }
        } else {
            UnifRecordRows::Concrete {
                rrows: RecordRowsF::Empty,
                var_levels_data: VarLevelsData::new_no_uvars(),
            }
        };

        if let RecordPatternTail::Capture(rest) = self.tail {
            bindings.push((rest, UnifType::concrete(TypeF::Record(tail.clone()))));
        }

        self.patterns
            .iter()
            .map(|field_pat| field_pat.pattern_types_inj(bindings, state, ctxt, mode))
            .try_fold(tail, |tail, row: Result<UnifRecordRow, TypecheckError>| {
                Ok(UnifRecordRows::concrete(RecordRowsF::Extend {
                    row: row?,
                    tail: Box::new(tail),
                }))
            })
    }
}

impl PatternTypes for Pattern {
    type PatType = UnifType;

    fn pattern_types_inj(
        &self,
        bindings: &mut Vec<(LocIdent, UnifType)>,
        state: &mut State,
        ctxt: &Context,
        mode: TypecheckMode,
    ) -> Result<Self::PatType, TypecheckError> {
        let typ = self.data.pattern_types_inj(bindings, state, ctxt, mode)?;

        if let Some(alias) = self.alias {
            bindings.push((alias, typ.clone()));
        }

        Ok(typ)
    }
}

impl PatternTypes for PatternData {
    type PatType = UnifType;

    fn pattern_types_inj(
        &self,
        bindings: &mut Vec<(LocIdent, UnifType)>,
        state: &mut State,
        ctxt: &Context,
        mode: TypecheckMode,
    ) -> Result<Self::PatType, TypecheckError> {
        match self {
            PatternData::Any(id) => {
                let typ = match mode {
                    TypecheckMode::Walk => mk_uniftype::dynamic(),
                    TypecheckMode::Enforce => state.table.fresh_type_uvar(ctxt.var_level),
                };

                bindings.push((*id, typ.clone()));

                Ok(typ)
            }
            PatternData::Record(record_pat) => Ok(UnifType::concrete(TypeF::Record(
                record_pat.pattern_types_inj(bindings, state, ctxt, mode)?,
            ))),
            PatternData::Enum(enum_pat) => {
                let row = enum_pat.pattern_types_inj(bindings, state, ctxt, mode)?;

                // This represents the single-row, closed type `[| row |]`
                Ok(UnifType::concrete(TypeF::Enum(UnifEnumRows::concrete(
                    EnumRowsF::Extend {
                        row,
                        tail: Box::new(UnifEnumRows::concrete(EnumRowsF::Empty)),
                    },
                ))))
            }
        }
    }
}

impl PatternTypes for FieldPattern {
    type PatType = UnifRecordRow;

    fn pattern_types_inj(
        &self,
        bindings: &mut Vec<(LocIdent, UnifType)>,
        state: &mut State,
        ctxt: &Context,
        mode: TypecheckMode,
    ) -> Result<Self::PatType, TypecheckError> {
        // If there is a static type annotations in a nested record patterns then we need to unify
        // them with the pattern type we've built to ensure (1) that they're mutually compatible
        // and (2) that we assign the annotated types to the right unification variables.
        let ty_row = match (
            &self.extra.metadata.annotation.typ,
            &self.pattern.data,
            mode,
        ) {
            // However, in walk mode, we only do that when the nested pattern isn't a leaf (i.e.
            // `Any`) for backward-compatibility reasons.
            //
            // Before this function was refactored, Nickel has been allowing things like `let {foo
            // : Number} = {foo = 1} in foo` in walk mode, which would fail to typecheck with the
            // generic approach: the pattern is parsed as `{foo : Number = foo}`, the second
            // occurrence of `foo` gets type `Dyn` in walk mode, but `Dyn` fails to unify with
            // `Number`. In this case, we don't recursively call `pattern_types_inj` in the first
            // place and just declare that the type of `foo` is `Number`.
            //
            // This special case should probably be ruled out, requiring the users to use `let {foo
            // | Number}` instead, at least outside of a statically typed code block. But before
            // this happens, we special case the old behavior and eschew unification.
            (Some(annot_ty), PatternData::Any(id), TypecheckMode::Walk) => {
                let ty_row = UnifType::from_type(annot_ty.typ.clone(), &ctxt.term_env);
                bindings.push((*id, ty_row.clone()));
                ty_row
            }
            (Some(annot_ty), _, _) => {
                let pos = annot_ty.typ.pos;
                let annot_uty = UnifType::from_type(annot_ty.typ.clone(), &ctxt.term_env);

                let ty_row = self
                    .pattern
                    .pattern_types_inj(bindings, state, ctxt, mode)?;

                ty_row
                    .clone()
                    .unify(annot_uty, state, ctxt)
                    .map_err(|e| e.into_typecheck_err(state, pos))?;

                ty_row
            }
            _ => self
                .pattern
                .pattern_types_inj(bindings, state, ctxt, mode)?,
        };

        Ok(UnifRecordRow {
            id: self.matched_id,
            typ: Box::new(ty_row),
        })
    }
}

impl PatternTypes for EnumPattern {
    type PatType = UnifEnumRow;

    fn pattern_types_inj(
        &self,
        bindings: &mut Vec<(LocIdent, UnifType)>,
        state: &mut State,
        ctxt: &Context,
        mode: TypecheckMode,
    ) -> Result<Self::PatType, TypecheckError> {
        let typ_arg = self
            .pattern
            .as_ref()
            .map(|pat| pat.pattern_types_inj(bindings, state, ctxt, mode))
            .transpose()?
            .map(Box::new);

        Ok(UnifEnumRow {
            id: self.tag,
            typ: typ_arg,
        })
    }
}