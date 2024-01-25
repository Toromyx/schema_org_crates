/// <https://schema.org/tissueSample>
pub const TISSUE_SAMPLE_PROPERTY_IRI_HTTP: &str = "http://schema.org/tissueSample";
/// <https://schema.org/tissueSample>
pub const TISSUE_SAMPLE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/tissueSample";
/// <https://schema.org/tissueSample>
pub const TISSUE_SAMPLE_PROPERTY_LABEL: &str = "tissueSample";
pub struct TissueSamplePropertyIri;
impl PartialEq<&str> for TissueSamplePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TISSUE_SAMPLE_PROPERTY_IRI_HTTP || *other == TISSUE_SAMPLE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TissueSamplePropertyIri> for &str {
	fn eq(&self, other: &TissueSamplePropertyIri) -> bool {
		*self == TISSUE_SAMPLE_PROPERTY_IRI_HTTP || *self == TISSUE_SAMPLE_PROPERTY_IRI_HTTPS
	}
}
pub struct TissueSamplePropertyIriOrLabel;
impl PartialEq<&str> for TissueSamplePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TissueSamplePropertyIri || *other == TISSUE_SAMPLE_PROPERTY_LABEL
	}
}
impl PartialEq<TissueSamplePropertyIriOrLabel> for &str {
	fn eq(&self, other: &TissueSamplePropertyIriOrLabel) -> bool {
		*self == TissueSamplePropertyIri || *self == TISSUE_SAMPLE_PROPERTY_LABEL
	}
}
