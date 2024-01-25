/// <https://schema.org/duplicateTherapy>
pub const DUPLICATE_THERAPY_PROPERTY_IRI_HTTP: &str = "http://schema.org/duplicateTherapy";
/// <https://schema.org/duplicateTherapy>
pub const DUPLICATE_THERAPY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/duplicateTherapy";
/// <https://schema.org/duplicateTherapy>
pub const DUPLICATE_THERAPY_PROPERTY_LABEL: &str = "duplicateTherapy";
pub struct DuplicateTherapyPropertyIri;
impl PartialEq<&str> for DuplicateTherapyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DUPLICATE_THERAPY_PROPERTY_IRI_HTTP
			|| *other == DUPLICATE_THERAPY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DuplicateTherapyPropertyIri> for &str {
	fn eq(&self, other: &DuplicateTherapyPropertyIri) -> bool {
		*self == DUPLICATE_THERAPY_PROPERTY_IRI_HTTP
			|| *self == DUPLICATE_THERAPY_PROPERTY_IRI_HTTPS
	}
}
pub struct DuplicateTherapyPropertyIriOrLabel;
impl PartialEq<&str> for DuplicateTherapyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DuplicateTherapyPropertyIri || *other == DUPLICATE_THERAPY_PROPERTY_LABEL
	}
}
impl PartialEq<DuplicateTherapyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DuplicateTherapyPropertyIriOrLabel) -> bool {
		*self == DuplicateTherapyPropertyIri || *self == DUPLICATE_THERAPY_PROPERTY_LABEL
	}
}
