/// <https://schema.org/acquiredFrom>
pub const ACQUIRED_FROM_PROPERTY_IRI_HTTP: &str = "http://schema.org/acquiredFrom";
/// <https://schema.org/acquiredFrom>
pub const ACQUIRED_FROM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/acquiredFrom";
/// <https://schema.org/acquiredFrom>
pub const ACQUIRED_FROM_PROPERTY_LABEL: &str = "acquiredFrom";
pub struct AcquiredFromPropertyIri;
impl PartialEq<&str> for AcquiredFromPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACQUIRED_FROM_PROPERTY_IRI_HTTP || *other == ACQUIRED_FROM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AcquiredFromPropertyIri> for &str {
	fn eq(&self, other: &AcquiredFromPropertyIri) -> bool {
		*self == ACQUIRED_FROM_PROPERTY_IRI_HTTP || *self == ACQUIRED_FROM_PROPERTY_IRI_HTTPS
	}
}
pub struct AcquiredFromPropertyIriOrLabel;
impl PartialEq<&str> for AcquiredFromPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AcquiredFromPropertyIri || *other == ACQUIRED_FROM_PROPERTY_LABEL
	}
}
impl PartialEq<AcquiredFromPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AcquiredFromPropertyIriOrLabel) -> bool {
		*self == AcquiredFromPropertyIri || *self == ACQUIRED_FROM_PROPERTY_LABEL
	}
}
