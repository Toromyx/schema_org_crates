/// <https://schema.org/domainIncludes>
pub const DOMAIN_INCLUDES_PROPERTY_IRI_HTTP: &str = "http://schema.org/domainIncludes";
/// <https://schema.org/domainIncludes>
pub const DOMAIN_INCLUDES_PROPERTY_IRI_HTTPS: &str = "https://schema.org/domainIncludes";
/// <https://schema.org/domainIncludes>
pub const DOMAIN_INCLUDES_PROPERTY_LABEL: &str = "domainIncludes";
pub struct DomainIncludesPropertyIri;
impl PartialEq<&str> for DomainIncludesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DOMAIN_INCLUDES_PROPERTY_IRI_HTTP || *other == DOMAIN_INCLUDES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DomainIncludesPropertyIri> for &str {
	fn eq(&self, other: &DomainIncludesPropertyIri) -> bool {
		*self == DOMAIN_INCLUDES_PROPERTY_IRI_HTTP || *self == DOMAIN_INCLUDES_PROPERTY_IRI_HTTPS
	}
}
pub struct DomainIncludesPropertyIriOrLabel;
impl PartialEq<&str> for DomainIncludesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DomainIncludesPropertyIri || *other == DOMAIN_INCLUDES_PROPERTY_LABEL
	}
}
impl PartialEq<DomainIncludesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DomainIncludesPropertyIriOrLabel) -> bool {
		*self == DomainIncludesPropertyIri || *self == DOMAIN_INCLUDES_PROPERTY_LABEL
	}
}
