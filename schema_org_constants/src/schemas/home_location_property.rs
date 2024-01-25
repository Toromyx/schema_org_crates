/// <https://schema.org/homeLocation>
pub const HOME_LOCATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/homeLocation";
/// <https://schema.org/homeLocation>
pub const HOME_LOCATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/homeLocation";
/// <https://schema.org/homeLocation>
pub const HOME_LOCATION_PROPERTY_LABEL: &str = "homeLocation";
pub struct HomeLocationPropertyIri;
impl PartialEq<&str> for HomeLocationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HOME_LOCATION_PROPERTY_IRI_HTTP || *other == HOME_LOCATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HomeLocationPropertyIri> for &str {
	fn eq(&self, other: &HomeLocationPropertyIri) -> bool {
		*self == HOME_LOCATION_PROPERTY_IRI_HTTP || *self == HOME_LOCATION_PROPERTY_IRI_HTTPS
	}
}
pub struct HomeLocationPropertyIriOrLabel;
impl PartialEq<&str> for HomeLocationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HomeLocationPropertyIri || *other == HOME_LOCATION_PROPERTY_LABEL
	}
}
impl PartialEq<HomeLocationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HomeLocationPropertyIriOrLabel) -> bool {
		*self == HomeLocationPropertyIri || *self == HOME_LOCATION_PROPERTY_LABEL
	}
}
