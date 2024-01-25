/// <https://schema.org/doseValue>
pub const DOSE_VALUE_PROPERTY_IRI_HTTP: &str = "http://schema.org/doseValue";
/// <https://schema.org/doseValue>
pub const DOSE_VALUE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/doseValue";
/// <https://schema.org/doseValue>
pub const DOSE_VALUE_PROPERTY_LABEL: &str = "doseValue";
pub struct DoseValuePropertyIri;
impl PartialEq<&str> for DoseValuePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DOSE_VALUE_PROPERTY_IRI_HTTP || *other == DOSE_VALUE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DoseValuePropertyIri> for &str {
	fn eq(&self, other: &DoseValuePropertyIri) -> bool {
		*self == DOSE_VALUE_PROPERTY_IRI_HTTP || *self == DOSE_VALUE_PROPERTY_IRI_HTTPS
	}
}
pub struct DoseValuePropertyIriOrLabel;
impl PartialEq<&str> for DoseValuePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DoseValuePropertyIri || *other == DOSE_VALUE_PROPERTY_LABEL
	}
}
impl PartialEq<DoseValuePropertyIriOrLabel> for &str {
	fn eq(&self, other: &DoseValuePropertyIriOrLabel) -> bool {
		*self == DoseValuePropertyIri || *self == DOSE_VALUE_PROPERTY_LABEL
	}
}
