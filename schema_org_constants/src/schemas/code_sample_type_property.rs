/// <https://schema.org/codeSampleType>
pub const CODE_SAMPLE_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/codeSampleType";
/// <https://schema.org/codeSampleType>
pub const CODE_SAMPLE_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/codeSampleType";
/// <https://schema.org/codeSampleType>
pub const CODE_SAMPLE_TYPE_PROPERTY_LABEL: &str = "codeSampleType";
pub struct CodeSampleTypePropertyIri;
impl PartialEq<&str> for CodeSampleTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CODE_SAMPLE_TYPE_PROPERTY_IRI_HTTP
			|| *other == CODE_SAMPLE_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CodeSampleTypePropertyIri> for &str {
	fn eq(&self, other: &CodeSampleTypePropertyIri) -> bool {
		*self == CODE_SAMPLE_TYPE_PROPERTY_IRI_HTTP || *self == CODE_SAMPLE_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct CodeSampleTypePropertyIriOrLabel;
impl PartialEq<&str> for CodeSampleTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CodeSampleTypePropertyIri || *other == CODE_SAMPLE_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<CodeSampleTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &CodeSampleTypePropertyIriOrLabel) -> bool {
		*self == CodeSampleTypePropertyIri || *self == CODE_SAMPLE_TYPE_PROPERTY_LABEL
	}
}
