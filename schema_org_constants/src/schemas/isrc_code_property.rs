/// <https://schema.org/isrcCode>
pub const ISRC_CODE_PROPERTY_IRI_HTTP: &str = "http://schema.org/isrcCode";
/// <https://schema.org/isrcCode>
pub const ISRC_CODE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/isrcCode";
/// <https://schema.org/isrcCode>
pub const ISRC_CODE_PROPERTY_LABEL: &str = "isrcCode";
pub struct IsrcCodePropertyIri;
impl PartialEq<&str> for IsrcCodePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ISRC_CODE_PROPERTY_IRI_HTTP || *other == ISRC_CODE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IsrcCodePropertyIri> for &str {
	fn eq(&self, other: &IsrcCodePropertyIri) -> bool {
		*self == ISRC_CODE_PROPERTY_IRI_HTTP || *self == ISRC_CODE_PROPERTY_IRI_HTTPS
	}
}
pub struct IsrcCodePropertyIriOrLabel;
impl PartialEq<&str> for IsrcCodePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IsrcCodePropertyIri || *other == ISRC_CODE_PROPERTY_LABEL
	}
}
impl PartialEq<IsrcCodePropertyIriOrLabel> for &str {
	fn eq(&self, other: &IsrcCodePropertyIriOrLabel) -> bool {
		*self == IsrcCodePropertyIri || *self == ISRC_CODE_PROPERTY_LABEL
	}
}
