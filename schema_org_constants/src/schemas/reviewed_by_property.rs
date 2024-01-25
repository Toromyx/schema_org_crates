/// <https://schema.org/reviewedBy>
pub const REVIEWED_BY_PROPERTY_IRI_HTTP: &str = "http://schema.org/reviewedBy";
/// <https://schema.org/reviewedBy>
pub const REVIEWED_BY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/reviewedBy";
/// <https://schema.org/reviewedBy>
pub const REVIEWED_BY_PROPERTY_LABEL: &str = "reviewedBy";
pub struct ReviewedByPropertyIri;
impl PartialEq<&str> for ReviewedByPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REVIEWED_BY_PROPERTY_IRI_HTTP || *other == REVIEWED_BY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ReviewedByPropertyIri> for &str {
	fn eq(&self, other: &ReviewedByPropertyIri) -> bool {
		*self == REVIEWED_BY_PROPERTY_IRI_HTTP || *self == REVIEWED_BY_PROPERTY_IRI_HTTPS
	}
}
pub struct ReviewedByPropertyIriOrLabel;
impl PartialEq<&str> for ReviewedByPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReviewedByPropertyIri || *other == REVIEWED_BY_PROPERTY_LABEL
	}
}
impl PartialEq<ReviewedByPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ReviewedByPropertyIriOrLabel) -> bool {
		*self == ReviewedByPropertyIri || *self == REVIEWED_BY_PROPERTY_LABEL
	}
}
