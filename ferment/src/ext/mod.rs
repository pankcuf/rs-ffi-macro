mod constraints;
mod nesting;
mod conversion;
mod composition;
mod merge;
mod scope;
mod collection;
mod item_helper;
mod local_connections;
mod visiting;
mod join;
mod pop;
mod mangle;
mod accessory;

pub use self::accessory::Accessory;
pub use self::collection::ScopeCollection;
pub use self::conversion::Conversion;
pub use self::mangle::{Mangle, ManglingRules};
pub use self::nesting::NestingExtension;
pub use self::constraints::Constraints;
pub use self::item_helper::ItemHelper;
pub use self::join::Join;
pub use self::merge::HashMapMergePolicy;
pub use self::merge::MergeInto;
pub use self::merge::MergePolicy;
pub use self::merge::ValueReplaceScenario;
pub use self::pop::Pop;
pub use self::visiting::add_trait_names;
pub use self::visiting::create_generics_chain;
pub use self::visiting::extract_trait_names;
pub use self::visiting::Visiting;