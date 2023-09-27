use super::*;
/// <https://schema.org/gameServer>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum GameServerProperty {
    #[cfg(any(
        any(feature = "game-server-schema", feature = "general-schema-section"),
        doc
    ))]
    GameServer(GameServer),
}
