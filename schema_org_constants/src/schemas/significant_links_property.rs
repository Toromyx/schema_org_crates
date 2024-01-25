/// <https://schema.org/significantLinks>
#[deprecated = "This schema is superseded by <https://schema.org/significantLink>."]
pub const SIGNIFICANT_LINKS_PROPERTY_IRI_HTTP: &str = "http://schema.org/significantLinks";
/// <https://schema.org/significantLinks>
#[deprecated = "This schema is superseded by <https://schema.org/significantLink>."]
pub const SIGNIFICANT_LINKS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/significantLinks";
/// <https://schema.org/significantLinks>
#[deprecated = "This schema is superseded by <https://schema.org/significantLink>."]
pub const SIGNIFICANT_LINKS_PROPERTY_LABEL: &str = "significantLinks";
pub struct SignificantLinksPropertyIri;
impl PartialEq<&str> for SignificantLinksPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SIGNIFICANT_LINKS_PROPERTY_IRI_HTTP
			|| *other == SIGNIFICANT_LINKS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SignificantLinksPropertyIri> for &str {
	fn eq(&self, other: &SignificantLinksPropertyIri) -> bool {
		*self == SIGNIFICANT_LINKS_PROPERTY_IRI_HTTP
			|| *self == SIGNIFICANT_LINKS_PROPERTY_IRI_HTTPS
	}
}
pub struct SignificantLinksPropertyIriOrLabel;
impl PartialEq<&str> for SignificantLinksPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SignificantLinksPropertyIri || *other == SIGNIFICANT_LINKS_PROPERTY_LABEL
	}
}
impl PartialEq<SignificantLinksPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SignificantLinksPropertyIriOrLabel) -> bool {
		*self == SignificantLinksPropertyIri || *self == SIGNIFICANT_LINKS_PROPERTY_LABEL
	}
}
