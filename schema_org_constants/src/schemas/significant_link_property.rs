/// <https://schema.org/significantLink>
pub const SIGNIFICANT_LINK_PROPERTY_IRI_HTTP: &str = "http://schema.org/significantLink";
/// <https://schema.org/significantLink>
pub const SIGNIFICANT_LINK_PROPERTY_IRI_HTTPS: &str = "https://schema.org/significantLink";
/// <https://schema.org/significantLink>
pub const SIGNIFICANT_LINK_PROPERTY_LABEL: &str = "significantLink";
pub struct SignificantLinkPropertyIri;
impl PartialEq<&str> for SignificantLinkPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SIGNIFICANT_LINK_PROPERTY_IRI_HTTP
			|| *other == SIGNIFICANT_LINK_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SignificantLinkPropertyIri> for &str {
	fn eq(&self, other: &SignificantLinkPropertyIri) -> bool {
		*self == SIGNIFICANT_LINK_PROPERTY_IRI_HTTP || *self == SIGNIFICANT_LINK_PROPERTY_IRI_HTTPS
	}
}
pub struct SignificantLinkPropertyIriOrLabel;
impl PartialEq<&str> for SignificantLinkPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SignificantLinkPropertyIri || *other == SIGNIFICANT_LINK_PROPERTY_LABEL
	}
}
impl PartialEq<SignificantLinkPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SignificantLinkPropertyIriOrLabel) -> bool {
		*self == SignificantLinkPropertyIri || *self == SIGNIFICANT_LINK_PROPERTY_LABEL
	}
}
