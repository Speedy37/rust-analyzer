//! FIXME: write short doc here

pub use hir_def::db::{
    BodyQuery, BodyWithSourceMapQuery, ComputeCrateDefMapQuery, ConstDataQuery,
    CrateLangItemsQuery, DefDatabase, DefDatabaseStorage, DocumentationQuery, EnumDataQuery,
    ExprScopesQuery, FunctionDataQuery, GenericParamsQuery, ImplDataQuery, InternDatabase,
    InternDatabaseStorage, LangItemQuery, ModuleLangItemsQuery, RawItemsQuery, StaticDataQuery,
    StructDataQuery, TraitDataQuery, TypeAliasDataQuery,
};
pub use hir_expand::db::{
    AstDatabase, AstDatabaseStorage, AstIdMapQuery, MacroArgQuery, MacroDefQuery, MacroExpandQuery,
    ParseMacroQuery,
};
pub use hir_ty::db::{
    AssociatedTyDataQuery, CallableItemSignatureQuery, DoInferQuery, FieldTypesQuery,
    GenericDefaultsQuery, GenericPredicatesQuery, HirDatabase, HirDatabaseStorage, ImplDatumQuery,
    ImplsForTraitQuery, ImplsInCrateQuery, StructDatumQuery, TraitDatumQuery, TraitSolveQuery,
    TyQuery, ValueTyQuery,
};

#[test]
fn hir_database_is_object_safe() {
    fn _assert_object_safe(_: &dyn HirDatabase) {}
}
