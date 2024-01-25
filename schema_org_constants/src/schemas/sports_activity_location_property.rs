/// <https://schema.org/sportsActivityLocation>
pub const SPORTS_ACTIVITY_LOCATION_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/sportsActivityLocation";
/// <https://schema.org/sportsActivityLocation>
pub const SPORTS_ACTIVITY_LOCATION_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/sportsActivityLocation";
/// <https://schema.org/sportsActivityLocation>
pub const SPORTS_ACTIVITY_LOCATION_PROPERTY_LABEL: &str = "sportsActivityLocation";
pub struct SportsActivityLocationPropertyIri;
impl PartialEq<&str> for SportsActivityLocationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SPORTS_ACTIVITY_LOCATION_PROPERTY_IRI_HTTP
			|| *other == SPORTS_ACTIVITY_LOCATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SportsActivityLocationPropertyIri> for &str {
	fn eq(&self, other: &SportsActivityLocationPropertyIri) -> bool {
		*self == SPORTS_ACTIVITY_LOCATION_PROPERTY_IRI_HTTP
			|| *self == SPORTS_ACTIVITY_LOCATION_PROPERTY_IRI_HTTPS
	}
}
pub struct SportsActivityLocationPropertyIriOrLabel;
impl PartialEq<&str> for SportsActivityLocationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SportsActivityLocationPropertyIri
			|| *other == SPORTS_ACTIVITY_LOCATION_PROPERTY_LABEL
	}
}
impl PartialEq<SportsActivityLocationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SportsActivityLocationPropertyIriOrLabel) -> bool {
		*self == SportsActivityLocationPropertyIri
			|| *self == SPORTS_ACTIVITY_LOCATION_PROPERTY_LABEL
	}
}
