/// <https://schema.org/Attorney>
pub const ATTORNEY_IRI_HTTP: &str = "http://schema.org/Attorney";
/// <https://schema.org/Attorney>
pub const ATTORNEY_IRI_HTTPS: &str = "https://schema.org/Attorney";
/// <https://schema.org/Attorney>
pub const ATTORNEY_LABEL: &str = "Attorney";
pub struct AttorneyIri;
impl PartialEq<&str> for AttorneyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ATTORNEY_IRI_HTTP || *other == ATTORNEY_IRI_HTTPS
	}
}
impl PartialEq<AttorneyIri> for &str {
	fn eq(&self, other: &AttorneyIri) -> bool {
		*self == ATTORNEY_IRI_HTTP || *self == ATTORNEY_IRI_HTTPS
	}
}
pub struct AttorneyIriOrLabel;
impl PartialEq<&str> for AttorneyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AttorneyIri || *other == ATTORNEY_LABEL
	}
}
impl PartialEq<AttorneyIriOrLabel> for &str {
	fn eq(&self, other: &AttorneyIriOrLabel) -> bool {
		*self == AttorneyIri || *self == ATTORNEY_LABEL
	}
}
