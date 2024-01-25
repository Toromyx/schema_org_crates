/// <https://schema.org/PerformingGroup>
pub const PERFORMING_GROUP_IRI_HTTP: &str = "http://schema.org/PerformingGroup";
/// <https://schema.org/PerformingGroup>
pub const PERFORMING_GROUP_IRI_HTTPS: &str = "https://schema.org/PerformingGroup";
/// <https://schema.org/PerformingGroup>
pub const PERFORMING_GROUP_LABEL: &str = "PerformingGroup";
pub struct PerformingGroupIri;
impl PartialEq<&str> for PerformingGroupIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PERFORMING_GROUP_IRI_HTTP || *other == PERFORMING_GROUP_IRI_HTTPS
	}
}
impl PartialEq<PerformingGroupIri> for &str {
	fn eq(&self, other: &PerformingGroupIri) -> bool {
		*self == PERFORMING_GROUP_IRI_HTTP || *self == PERFORMING_GROUP_IRI_HTTPS
	}
}
pub struct PerformingGroupIriOrLabel;
impl PartialEq<&str> for PerformingGroupIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PerformingGroupIri || *other == PERFORMING_GROUP_LABEL
	}
}
impl PartialEq<PerformingGroupIriOrLabel> for &str {
	fn eq(&self, other: &PerformingGroupIriOrLabel) -> bool {
		*self == PerformingGroupIri || *self == PERFORMING_GROUP_LABEL
	}
}
