/// <https://schema.org/supportingData>
pub const SUPPORTING_DATA_PROPERTY_IRI_HTTP: &str = "http://schema.org/supportingData";
/// <https://schema.org/supportingData>
pub const SUPPORTING_DATA_PROPERTY_IRI_HTTPS: &str = "https://schema.org/supportingData";
/// <https://schema.org/supportingData>
pub const SUPPORTING_DATA_PROPERTY_LABEL: &str = "supportingData";
pub struct SupportingDataPropertyIri;
impl PartialEq<&str> for SupportingDataPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SUPPORTING_DATA_PROPERTY_IRI_HTTP || *other == SUPPORTING_DATA_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SupportingDataPropertyIri> for &str {
	fn eq(&self, other: &SupportingDataPropertyIri) -> bool {
		*self == SUPPORTING_DATA_PROPERTY_IRI_HTTP || *self == SUPPORTING_DATA_PROPERTY_IRI_HTTPS
	}
}
pub struct SupportingDataPropertyIriOrLabel;
impl PartialEq<&str> for SupportingDataPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SupportingDataPropertyIri || *other == SUPPORTING_DATA_PROPERTY_LABEL
	}
}
impl PartialEq<SupportingDataPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SupportingDataPropertyIriOrLabel) -> bool {
		*self == SupportingDataPropertyIri || *self == SUPPORTING_DATA_PROPERTY_LABEL
	}
}
