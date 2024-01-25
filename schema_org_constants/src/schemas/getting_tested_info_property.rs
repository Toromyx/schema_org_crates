/// <https://schema.org/gettingTestedInfo>
pub const GETTING_TESTED_INFO_PROPERTY_IRI_HTTP: &str = "http://schema.org/gettingTestedInfo";
/// <https://schema.org/gettingTestedInfo>
pub const GETTING_TESTED_INFO_PROPERTY_IRI_HTTPS: &str = "https://schema.org/gettingTestedInfo";
/// <https://schema.org/gettingTestedInfo>
pub const GETTING_TESTED_INFO_PROPERTY_LABEL: &str = "gettingTestedInfo";
pub struct GettingTestedInfoPropertyIri;
impl PartialEq<&str> for GettingTestedInfoPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GETTING_TESTED_INFO_PROPERTY_IRI_HTTP
			|| *other == GETTING_TESTED_INFO_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<GettingTestedInfoPropertyIri> for &str {
	fn eq(&self, other: &GettingTestedInfoPropertyIri) -> bool {
		*self == GETTING_TESTED_INFO_PROPERTY_IRI_HTTP
			|| *self == GETTING_TESTED_INFO_PROPERTY_IRI_HTTPS
	}
}
pub struct GettingTestedInfoPropertyIriOrLabel;
impl PartialEq<&str> for GettingTestedInfoPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GettingTestedInfoPropertyIri || *other == GETTING_TESTED_INFO_PROPERTY_LABEL
	}
}
impl PartialEq<GettingTestedInfoPropertyIriOrLabel> for &str {
	fn eq(&self, other: &GettingTestedInfoPropertyIriOrLabel) -> bool {
		*self == GettingTestedInfoPropertyIri || *self == GETTING_TESTED_INFO_PROPERTY_LABEL
	}
}
