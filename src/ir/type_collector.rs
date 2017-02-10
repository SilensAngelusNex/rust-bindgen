//! Collecting type items.

use super::context::BindgenContext;
use super::item::ItemSet;

/// Collect all the type items referenced by this item.
pub trait TypeCollector {
    /// If a particular type needs extra information beyond what it has in
    /// `self` and `context` to find its referenced type items, its
    /// implementation can define this associated type, forcing callers to pass
    /// the needed information through.
    type Extra;

    /// Add each type item referenced by `self` into the `types` set.
    fn collect_types(&self,
                     context: &BindgenContext,
                     types: &mut ItemSet,
                     extra: &Self::Extra);
}
