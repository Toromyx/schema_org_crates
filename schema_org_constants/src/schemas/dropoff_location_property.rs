/// <https://schema.org/dropoffLocation>
pub const DROPOFF_LOCATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/dropoffLocation";
/// <https://schema.org/dropoffLocation>
pub const DROPOFF_LOCATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/dropoffLocation";
/// <https://schema.org/dropoffLocation>
pub const DROPOFF_LOCATION_PROPERTY_LABEL: &str = "dropoffLocation";
pub struct DropoffLocationPropertyIri;
impl PartialEq<&str> for DropoffLocationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DROPOFF_LOCATION_PROPERTY_IRI_HTTP
			|| *other == DROPOFF_LOCATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DropoffLocationPropertyIri> for &str {
	fn eq(&self, other: &DropoffLocationPropertyIri) -> bool {
		*self == DROPOFF_LOCATION_PROPERTY_IRI_HTTP || *self == DROPOFF_LOCATION_PROPERTY_IRI_HTTPS
	}
}
pub struct DropoffLocationPropertyIriOrLabel;
impl PartialEq<&str> for DropoffLocationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DropoffLocationPropertyIri || *other == DROPOFF_LOCATION_PROPERTY_LABEL
	}
}
impl PartialEq<DropoffLocationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DropoffLocationPropertyIriOrLabel) -> bool {
		*self == DropoffLocationPropertyIri || *self == DROPOFF_LOCATION_PROPERTY_LABEL
	}
}
