/// <https://schema.org/foundingLocation>
pub const FOUNDING_LOCATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/foundingLocation";
/// <https://schema.org/foundingLocation>
pub const FOUNDING_LOCATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/foundingLocation";
/// <https://schema.org/foundingLocation>
pub const FOUNDING_LOCATION_PROPERTY_LABEL: &str = "foundingLocation";
pub struct FoundingLocationPropertyIri;
impl PartialEq<&str> for FoundingLocationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FOUNDING_LOCATION_PROPERTY_IRI_HTTP
			|| *other == FOUNDING_LOCATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<FoundingLocationPropertyIri> for &str {
	fn eq(&self, other: &FoundingLocationPropertyIri) -> bool {
		*self == FOUNDING_LOCATION_PROPERTY_IRI_HTTP
			|| *self == FOUNDING_LOCATION_PROPERTY_IRI_HTTPS
	}
}
pub struct FoundingLocationPropertyIriOrLabel;
impl PartialEq<&str> for FoundingLocationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FoundingLocationPropertyIri || *other == FOUNDING_LOCATION_PROPERTY_LABEL
	}
}
impl PartialEq<FoundingLocationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &FoundingLocationPropertyIriOrLabel) -> bool {
		*self == FoundingLocationPropertyIri || *self == FOUNDING_LOCATION_PROPERTY_LABEL
	}
}
