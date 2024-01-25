/// <https://schema.org/SportsActivityLocation>
pub const SPORTS_ACTIVITY_LOCATION_IRI_HTTP: &str = "http://schema.org/SportsActivityLocation";
/// <https://schema.org/SportsActivityLocation>
pub const SPORTS_ACTIVITY_LOCATION_IRI_HTTPS: &str = "https://schema.org/SportsActivityLocation";
/// <https://schema.org/SportsActivityLocation>
pub const SPORTS_ACTIVITY_LOCATION_LABEL: &str = "SportsActivityLocation";
pub struct SportsActivityLocationIri;
impl PartialEq<&str> for SportsActivityLocationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SPORTS_ACTIVITY_LOCATION_IRI_HTTP || *other == SPORTS_ACTIVITY_LOCATION_IRI_HTTPS
	}
}
impl PartialEq<SportsActivityLocationIri> for &str {
	fn eq(&self, other: &SportsActivityLocationIri) -> bool {
		*self == SPORTS_ACTIVITY_LOCATION_IRI_HTTP || *self == SPORTS_ACTIVITY_LOCATION_IRI_HTTPS
	}
}
pub struct SportsActivityLocationIriOrLabel;
impl PartialEq<&str> for SportsActivityLocationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SportsActivityLocationIri || *other == SPORTS_ACTIVITY_LOCATION_LABEL
	}
}
impl PartialEq<SportsActivityLocationIriOrLabel> for &str {
	fn eq(&self, other: &SportsActivityLocationIriOrLabel) -> bool {
		*self == SportsActivityLocationIri || *self == SPORTS_ACTIVITY_LOCATION_LABEL
	}
}
