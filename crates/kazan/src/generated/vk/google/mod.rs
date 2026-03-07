pub mod decorate_string;
pub mod display_timing;
pub mod hlsl_functionality1;
pub mod surfaceless_query;
pub mod user_type;
pub(super) mod defs {
    use super::*;
    pub use decorate_string::defs::*;
    pub use display_timing::defs::*;
    pub use hlsl_functionality1::defs::*;
    pub use surfaceless_query::defs::*;
    pub use user_type::defs::*;
}
