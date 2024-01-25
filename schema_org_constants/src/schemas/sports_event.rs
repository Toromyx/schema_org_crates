/// <https://schema.org/SportsEvent>
pub const SPORTS_EVENT_IRI_HTTP: &str = "http://schema.org/SportsEvent";
/// <https://schema.org/SportsEvent>
pub const SPORTS_EVENT_IRI_HTTPS: &str = "https://schema.org/SportsEvent";
/// <https://schema.org/SportsEvent>
pub const SPORTS_EVENT_LABEL: &str = "SportsEvent";
pub struct SportsEventIri;
impl PartialEq<&str> for SportsEventIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SPORTS_EVENT_IRI_HTTP || *other == SPORTS_EVENT_IRI_HTTPS
	}
}
impl PartialEq<SportsEventIri> for &str {
	fn eq(&self, other: &SportsEventIri) -> bool {
		*self == SPORTS_EVENT_IRI_HTTP || *self == SPORTS_EVENT_IRI_HTTPS
	}
}
pub struct SportsEventIriOrLabel;
impl PartialEq<&str> for SportsEventIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SportsEventIri || *other == SPORTS_EVENT_LABEL
	}
}
impl PartialEq<SportsEventIriOrLabel> for &str {
	fn eq(&self, other: &SportsEventIriOrLabel) -> bool {
		*self == SportsEventIri || *self == SPORTS_EVENT_LABEL
	}
}
