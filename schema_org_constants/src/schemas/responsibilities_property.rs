/// <https://schema.org/responsibilities>
pub const RESPONSIBILITIES_PROPERTY_IRI_HTTP: &str = "http://schema.org/responsibilities";
/// <https://schema.org/responsibilities>
pub const RESPONSIBILITIES_PROPERTY_IRI_HTTPS: &str = "https://schema.org/responsibilities";
/// <https://schema.org/responsibilities>
pub const RESPONSIBILITIES_PROPERTY_LABEL: &str = "responsibilities";
pub struct ResponsibilitiesPropertyIri;
impl PartialEq<&str> for ResponsibilitiesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RESPONSIBILITIES_PROPERTY_IRI_HTTP
			|| *other == RESPONSIBILITIES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ResponsibilitiesPropertyIri> for &str {
	fn eq(&self, other: &ResponsibilitiesPropertyIri) -> bool {
		*self == RESPONSIBILITIES_PROPERTY_IRI_HTTP || *self == RESPONSIBILITIES_PROPERTY_IRI_HTTPS
	}
}
pub struct ResponsibilitiesPropertyIriOrLabel;
impl PartialEq<&str> for ResponsibilitiesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ResponsibilitiesPropertyIri || *other == RESPONSIBILITIES_PROPERTY_LABEL
	}
}
impl PartialEq<ResponsibilitiesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ResponsibilitiesPropertyIriOrLabel) -> bool {
		*self == ResponsibilitiesPropertyIri || *self == RESPONSIBILITIES_PROPERTY_LABEL
	}
}
