/// <https://schema.org/OrganizeAction>
pub const ORGANIZE_ACTION_IRI_HTTP: &str = "http://schema.org/OrganizeAction";
/// <https://schema.org/OrganizeAction>
pub const ORGANIZE_ACTION_IRI_HTTPS: &str = "https://schema.org/OrganizeAction";
/// <https://schema.org/OrganizeAction>
pub const ORGANIZE_ACTION_LABEL: &str = "OrganizeAction";
pub struct OrganizeActionIri;
impl PartialEq<&str> for OrganizeActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ORGANIZE_ACTION_IRI_HTTP || *other == ORGANIZE_ACTION_IRI_HTTPS
	}
}
impl PartialEq<OrganizeActionIri> for &str {
	fn eq(&self, other: &OrganizeActionIri) -> bool {
		*self == ORGANIZE_ACTION_IRI_HTTP || *self == ORGANIZE_ACTION_IRI_HTTPS
	}
}
pub struct OrganizeActionIriOrLabel;
impl PartialEq<&str> for OrganizeActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OrganizeActionIri || *other == ORGANIZE_ACTION_LABEL
	}
}
impl PartialEq<OrganizeActionIriOrLabel> for &str {
	fn eq(&self, other: &OrganizeActionIriOrLabel) -> bool {
		*self == OrganizeActionIri || *self == ORGANIZE_ACTION_LABEL
	}
}
