/// <https://schema.org/iswcCode>
pub const ISWC_CODE_PROPERTY_IRI_HTTP: &str = "http://schema.org/iswcCode";
/// <https://schema.org/iswcCode>
pub const ISWC_CODE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/iswcCode";
/// <https://schema.org/iswcCode>
pub const ISWC_CODE_PROPERTY_LABEL: &str = "iswcCode";
pub struct IswcCodePropertyIri;
impl PartialEq<&str> for IswcCodePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ISWC_CODE_PROPERTY_IRI_HTTP || *other == ISWC_CODE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IswcCodePropertyIri> for &str {
	fn eq(&self, other: &IswcCodePropertyIri) -> bool {
		*self == ISWC_CODE_PROPERTY_IRI_HTTP || *self == ISWC_CODE_PROPERTY_IRI_HTTPS
	}
}
pub struct IswcCodePropertyIriOrLabel;
impl PartialEq<&str> for IswcCodePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IswcCodePropertyIri || *other == ISWC_CODE_PROPERTY_LABEL
	}
}
impl PartialEq<IswcCodePropertyIriOrLabel> for &str {
	fn eq(&self, other: &IswcCodePropertyIriOrLabel) -> bool {
		*self == IswcCodePropertyIri || *self == ISWC_CODE_PROPERTY_LABEL
	}
}
