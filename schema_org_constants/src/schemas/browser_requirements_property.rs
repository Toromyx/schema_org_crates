/// <https://schema.org/browserRequirements>
pub const BROWSER_REQUIREMENTS_PROPERTY_IRI_HTTP: &str = "http://schema.org/browserRequirements";
/// <https://schema.org/browserRequirements>
pub const BROWSER_REQUIREMENTS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/browserRequirements";
/// <https://schema.org/browserRequirements>
pub const BROWSER_REQUIREMENTS_PROPERTY_LABEL: &str = "browserRequirements";
pub struct BrowserRequirementsPropertyIri;
impl PartialEq<&str> for BrowserRequirementsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BROWSER_REQUIREMENTS_PROPERTY_IRI_HTTP
			|| *other == BROWSER_REQUIREMENTS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BrowserRequirementsPropertyIri> for &str {
	fn eq(&self, other: &BrowserRequirementsPropertyIri) -> bool {
		*self == BROWSER_REQUIREMENTS_PROPERTY_IRI_HTTP
			|| *self == BROWSER_REQUIREMENTS_PROPERTY_IRI_HTTPS
	}
}
pub struct BrowserRequirementsPropertyIriOrLabel;
impl PartialEq<&str> for BrowserRequirementsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BrowserRequirementsPropertyIri || *other == BROWSER_REQUIREMENTS_PROPERTY_LABEL
	}
}
impl PartialEq<BrowserRequirementsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BrowserRequirementsPropertyIriOrLabel) -> bool {
		*self == BrowserRequirementsPropertyIri || *self == BROWSER_REQUIREMENTS_PROPERTY_LABEL
	}
}
