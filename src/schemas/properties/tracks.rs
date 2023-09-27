use super::*;
/// <https://schema.org/tracks>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum TracksProperty {
    #[cfg(any(
        any(feature = "music-recording-schema", feature = "general-schema-section"),
        doc
    ))]
    MusicRecording(MusicRecording),
}
