pub use self::result::Result;
pub use self::behavior::{
    Behavior,
    ResultBehavior, EventBehavior,
    SequenceBehavior, SelectorBehavior
};

mod result;
mod behavior;