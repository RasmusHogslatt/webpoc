use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum MillType {
    #[default]
    SquareEnd,
    ChipSplitter,
    BallNose,
    LollyPop,
    HighFeed,
    Tapered,
    Deburring,
    TSlot,
    Engraving,
    MillingCutter,
    ExchangeableEnd,
}
