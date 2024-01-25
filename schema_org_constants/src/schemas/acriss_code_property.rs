/// <https://schema.org/acrissCode>
pub const ACRISS_CODE_PROPERTY_IRI_HTTP: &str = "http://schema.org/acrissCode";
/// <https://schema.org/acrissCode>
pub const ACRISS_CODE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/acrissCode";
/// <https://schema.org/acrissCode>
pub const ACRISS_CODE_PROPERTY_LABEL: &str = "acrissCode";
pub struct AcrissCodePropertyIri;
impl PartialEq<&str> for AcrissCodePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACRISS_CODE_PROPERTY_IRI_HTTP || *other == ACRISS_CODE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AcrissCodePropertyIri> for &str {
	fn eq(&self, other: &AcrissCodePropertyIri) -> bool {
		*self == ACRISS_CODE_PROPERTY_IRI_HTTP || *self == ACRISS_CODE_PROPERTY_IRI_HTTPS
	}
}
pub struct AcrissCodePropertyIriOrLabel;
impl PartialEq<&str> for AcrissCodePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AcrissCodePropertyIri || *other == ACRISS_CODE_PROPERTY_LABEL
	}
}
impl PartialEq<AcrissCodePropertyIriOrLabel> for &str {
	fn eq(&self, other: &AcrissCodePropertyIriOrLabel) -> bool {
		*self == AcrissCodePropertyIri || *self == ACRISS_CODE_PROPERTY_LABEL
	}
}
