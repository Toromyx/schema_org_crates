/// <https://schema.org/industry>
pub const INDUSTRY_PROPERTY_IRI_HTTP: &str = "http://schema.org/industry";
/// <https://schema.org/industry>
pub const INDUSTRY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/industry";
/// <https://schema.org/industry>
pub const INDUSTRY_PROPERTY_LABEL: &str = "industry";
pub struct IndustryPropertyIri;
impl PartialEq<&str> for IndustryPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INDUSTRY_PROPERTY_IRI_HTTP || *other == INDUSTRY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IndustryPropertyIri> for &str {
	fn eq(&self, other: &IndustryPropertyIri) -> bool {
		*self == INDUSTRY_PROPERTY_IRI_HTTP || *self == INDUSTRY_PROPERTY_IRI_HTTPS
	}
}
pub struct IndustryPropertyIriOrLabel;
impl PartialEq<&str> for IndustryPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IndustryPropertyIri || *other == INDUSTRY_PROPERTY_LABEL
	}
}
impl PartialEq<IndustryPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IndustryPropertyIriOrLabel) -> bool {
		*self == IndustryPropertyIri || *self == INDUSTRY_PROPERTY_LABEL
	}
}
