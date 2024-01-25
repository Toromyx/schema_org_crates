/// <https://schema.org/AllocateAction>
pub const ALLOCATE_ACTION_IRI_HTTP: &str = "http://schema.org/AllocateAction";
/// <https://schema.org/AllocateAction>
pub const ALLOCATE_ACTION_IRI_HTTPS: &str = "https://schema.org/AllocateAction";
/// <https://schema.org/AllocateAction>
pub const ALLOCATE_ACTION_LABEL: &str = "AllocateAction";
pub struct AllocateActionIri;
impl PartialEq<&str> for AllocateActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ALLOCATE_ACTION_IRI_HTTP || *other == ALLOCATE_ACTION_IRI_HTTPS
	}
}
impl PartialEq<AllocateActionIri> for &str {
	fn eq(&self, other: &AllocateActionIri) -> bool {
		*self == ALLOCATE_ACTION_IRI_HTTP || *self == ALLOCATE_ACTION_IRI_HTTPS
	}
}
pub struct AllocateActionIriOrLabel;
impl PartialEq<&str> for AllocateActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AllocateActionIri || *other == ALLOCATE_ACTION_LABEL
	}
}
impl PartialEq<AllocateActionIriOrLabel> for &str {
	fn eq(&self, other: &AllocateActionIriOrLabel) -> bool {
		*self == AllocateActionIri || *self == ALLOCATE_ACTION_LABEL
	}
}
