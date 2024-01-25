/// <https://schema.org/durationOfWarranty>
pub const DURATION_OF_WARRANTY_PROPERTY_IRI_HTTP: &str = "http://schema.org/durationOfWarranty";
/// <https://schema.org/durationOfWarranty>
pub const DURATION_OF_WARRANTY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/durationOfWarranty";
/// <https://schema.org/durationOfWarranty>
pub const DURATION_OF_WARRANTY_PROPERTY_LABEL: &str = "durationOfWarranty";
pub struct DurationOfWarrantyPropertyIri;
impl PartialEq<&str> for DurationOfWarrantyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DURATION_OF_WARRANTY_PROPERTY_IRI_HTTP
			|| *other == DURATION_OF_WARRANTY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DurationOfWarrantyPropertyIri> for &str {
	fn eq(&self, other: &DurationOfWarrantyPropertyIri) -> bool {
		*self == DURATION_OF_WARRANTY_PROPERTY_IRI_HTTP
			|| *self == DURATION_OF_WARRANTY_PROPERTY_IRI_HTTPS
	}
}
pub struct DurationOfWarrantyPropertyIriOrLabel;
impl PartialEq<&str> for DurationOfWarrantyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DurationOfWarrantyPropertyIri || *other == DURATION_OF_WARRANTY_PROPERTY_LABEL
	}
}
impl PartialEq<DurationOfWarrantyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DurationOfWarrantyPropertyIriOrLabel) -> bool {
		*self == DurationOfWarrantyPropertyIri || *self == DURATION_OF_WARRANTY_PROPERTY_LABEL
	}
}
