use super::*;
/// <https://schema.org/homeTeam>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum HomeTeamProperty {
    #[cfg(any(
        any(feature = "person-schema", feature = "general-schema-section"),
        doc
    ))]
    Person(Person),
    #[cfg(any(
        any(feature = "sports-team-schema", feature = "general-schema-section"),
        doc
    ))]
    SportsTeam(SportsTeam),
}
