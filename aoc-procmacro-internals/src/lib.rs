mod fetcher;
pub use fetcher::*;
pub mod public {
    pub use super::fetcher::{get_aoc_data, set_session, AocDataType};
}
