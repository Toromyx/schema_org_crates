/// <https://schema.org/DepartAction>
pub const DEPART_ACTION_IRI_HTTP: &str = "http://schema.org/DepartAction";
/// <https://schema.org/DepartAction>
pub const DEPART_ACTION_IRI_HTTPS: &str = "https://schema.org/DepartAction";
/// <https://schema.org/DepartAction>
pub const DEPART_ACTION_LABEL: &str = "DepartAction";
pub struct DepartActionIri;
impl PartialEq<&str> for DepartActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DEPART_ACTION_IRI_HTTP || *other == DEPART_ACTION_IRI_HTTPS
	}
}
impl PartialEq<DepartActionIri> for &str {
	fn eq(&self, other: &DepartActionIri) -> bool {
		*self == DEPART_ACTION_IRI_HTTP || *self == DEPART_ACTION_IRI_HTTPS
	}
}
pub struct DepartActionIriOrLabel;
impl PartialEq<&str> for DepartActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DepartActionIri || *other == DEPART_ACTION_LABEL
	}
}
impl PartialEq<DepartActionIriOrLabel> for &str {
	fn eq(&self, other: &DepartActionIriOrLabel) -> bool {
		*self == DepartActionIri || *self == DEPART_ACTION_LABEL
	}
}
