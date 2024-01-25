/// <https://schema.org/unnamedSourcesPolicy>
pub const UNNAMED_SOURCES_POLICY_PROPERTY_IRI_HTTP: &str = "http://schema.org/unnamedSourcesPolicy";
/// <https://schema.org/unnamedSourcesPolicy>
pub const UNNAMED_SOURCES_POLICY_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/unnamedSourcesPolicy";
/// <https://schema.org/unnamedSourcesPolicy>
pub const UNNAMED_SOURCES_POLICY_PROPERTY_LABEL: &str = "unnamedSourcesPolicy";
pub struct UnnamedSourcesPolicyPropertyIri;
impl PartialEq<&str> for UnnamedSourcesPolicyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == UNNAMED_SOURCES_POLICY_PROPERTY_IRI_HTTP
			|| *other == UNNAMED_SOURCES_POLICY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<UnnamedSourcesPolicyPropertyIri> for &str {
	fn eq(&self, other: &UnnamedSourcesPolicyPropertyIri) -> bool {
		*self == UNNAMED_SOURCES_POLICY_PROPERTY_IRI_HTTP
			|| *self == UNNAMED_SOURCES_POLICY_PROPERTY_IRI_HTTPS
	}
}
pub struct UnnamedSourcesPolicyPropertyIriOrLabel;
impl PartialEq<&str> for UnnamedSourcesPolicyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UnnamedSourcesPolicyPropertyIri || *other == UNNAMED_SOURCES_POLICY_PROPERTY_LABEL
	}
}
impl PartialEq<UnnamedSourcesPolicyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &UnnamedSourcesPolicyPropertyIriOrLabel) -> bool {
		*self == UnnamedSourcesPolicyPropertyIri || *self == UNNAMED_SOURCES_POLICY_PROPERTY_LABEL
	}
}
