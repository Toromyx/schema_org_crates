/// <https://schema.org/affiliation>
pub const AFFILIATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/affiliation";
/// <https://schema.org/affiliation>
pub const AFFILIATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/affiliation";
/// <https://schema.org/affiliation>
pub const AFFILIATION_PROPERTY_LABEL: &str = "affiliation";
pub struct AffiliationPropertyIri;
impl PartialEq<&str> for AffiliationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AFFILIATION_PROPERTY_IRI_HTTP || *other == AFFILIATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AffiliationPropertyIri> for &str {
	fn eq(&self, other: &AffiliationPropertyIri) -> bool {
		*self == AFFILIATION_PROPERTY_IRI_HTTP || *self == AFFILIATION_PROPERTY_IRI_HTTPS
	}
}
pub struct AffiliationPropertyIriOrLabel;
impl PartialEq<&str> for AffiliationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AffiliationPropertyIri || *other == AFFILIATION_PROPERTY_LABEL
	}
}
impl PartialEq<AffiliationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AffiliationPropertyIriOrLabel) -> bool {
		*self == AffiliationPropertyIri || *self == AFFILIATION_PROPERTY_LABEL
	}
}
