/// <https://schema.org/sampleType>
#[deprecated = "This schema is superseded by <https://schema.org/codeSampleType>."]
pub const SAMPLE_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/sampleType";
/// <https://schema.org/sampleType>
#[deprecated = "This schema is superseded by <https://schema.org/codeSampleType>."]
pub const SAMPLE_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/sampleType";
/// <https://schema.org/sampleType>
#[deprecated = "This schema is superseded by <https://schema.org/codeSampleType>."]
pub const SAMPLE_TYPE_PROPERTY_LABEL: &str = "sampleType";
pub struct SampleTypePropertyIri;
impl PartialEq<&str> for SampleTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SAMPLE_TYPE_PROPERTY_IRI_HTTP || *other == SAMPLE_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SampleTypePropertyIri> for &str {
	fn eq(&self, other: &SampleTypePropertyIri) -> bool {
		*self == SAMPLE_TYPE_PROPERTY_IRI_HTTP || *self == SAMPLE_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct SampleTypePropertyIriOrLabel;
impl PartialEq<&str> for SampleTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SampleTypePropertyIri || *other == SAMPLE_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<SampleTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &SampleTypePropertyIriOrLabel) -> bool {
		*self == SampleTypePropertyIri || *self == SAMPLE_TYPE_PROPERTY_LABEL
	}
}
