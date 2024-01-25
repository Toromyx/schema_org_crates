/// <https://schema.org/StadiumOrArena>
pub const STADIUM_OR_ARENA_IRI_HTTP: &str = "http://schema.org/StadiumOrArena";
/// <https://schema.org/StadiumOrArena>
pub const STADIUM_OR_ARENA_IRI_HTTPS: &str = "https://schema.org/StadiumOrArena";
/// <https://schema.org/StadiumOrArena>
pub const STADIUM_OR_ARENA_LABEL: &str = "StadiumOrArena";
pub struct StadiumOrArenaIri;
impl PartialEq<&str> for StadiumOrArenaIri {
	fn eq(&self, other: &&str) -> bool {
		*other == STADIUM_OR_ARENA_IRI_HTTP || *other == STADIUM_OR_ARENA_IRI_HTTPS
	}
}
impl PartialEq<StadiumOrArenaIri> for &str {
	fn eq(&self, other: &StadiumOrArenaIri) -> bool {
		*self == STADIUM_OR_ARENA_IRI_HTTP || *self == STADIUM_OR_ARENA_IRI_HTTPS
	}
}
pub struct StadiumOrArenaIriOrLabel;
impl PartialEq<&str> for StadiumOrArenaIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == StadiumOrArenaIri || *other == STADIUM_OR_ARENA_LABEL
	}
}
impl PartialEq<StadiumOrArenaIriOrLabel> for &str {
	fn eq(&self, other: &StadiumOrArenaIriOrLabel) -> bool {
		*self == StadiumOrArenaIri || *self == STADIUM_OR_ARENA_LABEL
	}
}
