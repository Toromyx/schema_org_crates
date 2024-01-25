/// <https://schema.org/subStageSuffix>
pub const SUB_STAGE_SUFFIX_PROPERTY_IRI_HTTP: &str = "http://schema.org/subStageSuffix";
/// <https://schema.org/subStageSuffix>
pub const SUB_STAGE_SUFFIX_PROPERTY_IRI_HTTPS: &str = "https://schema.org/subStageSuffix";
/// <https://schema.org/subStageSuffix>
pub const SUB_STAGE_SUFFIX_PROPERTY_LABEL: &str = "subStageSuffix";
pub struct SubStageSuffixPropertyIri;
impl PartialEq<&str> for SubStageSuffixPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SUB_STAGE_SUFFIX_PROPERTY_IRI_HTTP
			|| *other == SUB_STAGE_SUFFIX_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SubStageSuffixPropertyIri> for &str {
	fn eq(&self, other: &SubStageSuffixPropertyIri) -> bool {
		*self == SUB_STAGE_SUFFIX_PROPERTY_IRI_HTTP || *self == SUB_STAGE_SUFFIX_PROPERTY_IRI_HTTPS
	}
}
pub struct SubStageSuffixPropertyIriOrLabel;
impl PartialEq<&str> for SubStageSuffixPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SubStageSuffixPropertyIri || *other == SUB_STAGE_SUFFIX_PROPERTY_LABEL
	}
}
impl PartialEq<SubStageSuffixPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SubStageSuffixPropertyIriOrLabel) -> bool {
		*self == SubStageSuffixPropertyIri || *self == SUB_STAGE_SUFFIX_PROPERTY_LABEL
	}
}
