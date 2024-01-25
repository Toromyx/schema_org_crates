/// <https://schema.org/affectedBy>
pub const AFFECTED_BY_PROPERTY_IRI_HTTP: &str = "http://schema.org/affectedBy";
/// <https://schema.org/affectedBy>
pub const AFFECTED_BY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/affectedBy";
/// <https://schema.org/affectedBy>
pub const AFFECTED_BY_PROPERTY_LABEL: &str = "affectedBy";
pub struct AffectedByPropertyIri;
impl PartialEq<&str> for AffectedByPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AFFECTED_BY_PROPERTY_IRI_HTTP || *other == AFFECTED_BY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AffectedByPropertyIri> for &str {
	fn eq(&self, other: &AffectedByPropertyIri) -> bool {
		*self == AFFECTED_BY_PROPERTY_IRI_HTTP || *self == AFFECTED_BY_PROPERTY_IRI_HTTPS
	}
}
pub struct AffectedByPropertyIriOrLabel;
impl PartialEq<&str> for AffectedByPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AffectedByPropertyIri || *other == AFFECTED_BY_PROPERTY_LABEL
	}
}
impl PartialEq<AffectedByPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AffectedByPropertyIriOrLabel) -> bool {
		*self == AffectedByPropertyIri || *self == AFFECTED_BY_PROPERTY_LABEL
	}
}
