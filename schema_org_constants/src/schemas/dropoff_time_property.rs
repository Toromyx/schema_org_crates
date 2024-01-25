/// <https://schema.org/dropoffTime>
pub const DROPOFF_TIME_PROPERTY_IRI_HTTP: &str = "http://schema.org/dropoffTime";
/// <https://schema.org/dropoffTime>
pub const DROPOFF_TIME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/dropoffTime";
/// <https://schema.org/dropoffTime>
pub const DROPOFF_TIME_PROPERTY_LABEL: &str = "dropoffTime";
pub struct DropoffTimePropertyIri;
impl PartialEq<&str> for DropoffTimePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DROPOFF_TIME_PROPERTY_IRI_HTTP || *other == DROPOFF_TIME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DropoffTimePropertyIri> for &str {
	fn eq(&self, other: &DropoffTimePropertyIri) -> bool {
		*self == DROPOFF_TIME_PROPERTY_IRI_HTTP || *self == DROPOFF_TIME_PROPERTY_IRI_HTTPS
	}
}
pub struct DropoffTimePropertyIriOrLabel;
impl PartialEq<&str> for DropoffTimePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DropoffTimePropertyIri || *other == DROPOFF_TIME_PROPERTY_LABEL
	}
}
impl PartialEq<DropoffTimePropertyIriOrLabel> for &str {
	fn eq(&self, other: &DropoffTimePropertyIriOrLabel) -> bool {
		*self == DropoffTimePropertyIri || *self == DROPOFF_TIME_PROPERTY_LABEL
	}
}
