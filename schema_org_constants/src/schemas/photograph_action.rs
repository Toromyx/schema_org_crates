/// <https://schema.org/PhotographAction>
pub const PHOTOGRAPH_ACTION_IRI_HTTP: &str = "http://schema.org/PhotographAction";
/// <https://schema.org/PhotographAction>
pub const PHOTOGRAPH_ACTION_IRI_HTTPS: &str = "https://schema.org/PhotographAction";
/// <https://schema.org/PhotographAction>
pub const PHOTOGRAPH_ACTION_LABEL: &str = "PhotographAction";
pub struct PhotographActionIri;
impl PartialEq<&str> for PhotographActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PHOTOGRAPH_ACTION_IRI_HTTP || *other == PHOTOGRAPH_ACTION_IRI_HTTPS
	}
}
impl PartialEq<PhotographActionIri> for &str {
	fn eq(&self, other: &PhotographActionIri) -> bool {
		*self == PHOTOGRAPH_ACTION_IRI_HTTP || *self == PHOTOGRAPH_ACTION_IRI_HTTPS
	}
}
pub struct PhotographActionIriOrLabel;
impl PartialEq<&str> for PhotographActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PhotographActionIri || *other == PHOTOGRAPH_ACTION_LABEL
	}
}
impl PartialEq<PhotographActionIriOrLabel> for &str {
	fn eq(&self, other: &PhotographActionIriOrLabel) -> bool {
		*self == PhotographActionIri || *self == PHOTOGRAPH_ACTION_LABEL
	}
}
