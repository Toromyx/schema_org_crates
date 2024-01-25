/// <https://schema.org/leiCode>
pub const LEI_CODE_PROPERTY_IRI_HTTP: &str = "http://schema.org/leiCode";
/// <https://schema.org/leiCode>
pub const LEI_CODE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/leiCode";
/// <https://schema.org/leiCode>
pub const LEI_CODE_PROPERTY_LABEL: &str = "leiCode";
pub struct LeiCodePropertyIri;
impl PartialEq<&str> for LeiCodePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LEI_CODE_PROPERTY_IRI_HTTP || *other == LEI_CODE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LeiCodePropertyIri> for &str {
	fn eq(&self, other: &LeiCodePropertyIri) -> bool {
		*self == LEI_CODE_PROPERTY_IRI_HTTP || *self == LEI_CODE_PROPERTY_IRI_HTTPS
	}
}
pub struct LeiCodePropertyIriOrLabel;
impl PartialEq<&str> for LeiCodePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LeiCodePropertyIri || *other == LEI_CODE_PROPERTY_LABEL
	}
}
impl PartialEq<LeiCodePropertyIriOrLabel> for &str {
	fn eq(&self, other: &LeiCodePropertyIriOrLabel) -> bool {
		*self == LeiCodePropertyIri || *self == LEI_CODE_PROPERTY_LABEL
	}
}
