/// <https://schema.org/sportsEvent>
pub const SPORTS_EVENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/sportsEvent";
/// <https://schema.org/sportsEvent>
pub const SPORTS_EVENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/sportsEvent";
/// <https://schema.org/sportsEvent>
pub const SPORTS_EVENT_PROPERTY_LABEL: &str = "sportsEvent";
pub struct SportsEventPropertyIri;
impl PartialEq<&str> for SportsEventPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SPORTS_EVENT_PROPERTY_IRI_HTTP || *other == SPORTS_EVENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SportsEventPropertyIri> for &str {
	fn eq(&self, other: &SportsEventPropertyIri) -> bool {
		*self == SPORTS_EVENT_PROPERTY_IRI_HTTP || *self == SPORTS_EVENT_PROPERTY_IRI_HTTPS
	}
}
pub struct SportsEventPropertyIriOrLabel;
impl PartialEq<&str> for SportsEventPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SportsEventPropertyIri || *other == SPORTS_EVENT_PROPERTY_LABEL
	}
}
impl PartialEq<SportsEventPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SportsEventPropertyIriOrLabel) -> bool {
		*self == SportsEventPropertyIri || *self == SPORTS_EVENT_PROPERTY_LABEL
	}
}
