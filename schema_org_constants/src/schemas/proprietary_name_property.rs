/// <https://schema.org/proprietaryName>
pub const PROPRIETARY_NAME_PROPERTY_IRI_HTTP: &str = "http://schema.org/proprietaryName";
/// <https://schema.org/proprietaryName>
pub const PROPRIETARY_NAME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/proprietaryName";
/// <https://schema.org/proprietaryName>
pub const PROPRIETARY_NAME_PROPERTY_LABEL: &str = "proprietaryName";
pub struct ProprietaryNamePropertyIri;
impl PartialEq<&str> for ProprietaryNamePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PROPRIETARY_NAME_PROPERTY_IRI_HTTP
			|| *other == PROPRIETARY_NAME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ProprietaryNamePropertyIri> for &str {
	fn eq(&self, other: &ProprietaryNamePropertyIri) -> bool {
		*self == PROPRIETARY_NAME_PROPERTY_IRI_HTTP || *self == PROPRIETARY_NAME_PROPERTY_IRI_HTTPS
	}
}
pub struct ProprietaryNamePropertyIriOrLabel;
impl PartialEq<&str> for ProprietaryNamePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProprietaryNamePropertyIri || *other == PROPRIETARY_NAME_PROPERTY_LABEL
	}
}
impl PartialEq<ProprietaryNamePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ProprietaryNamePropertyIriOrLabel) -> bool {
		*self == ProprietaryNamePropertyIri || *self == PROPRIETARY_NAME_PROPERTY_LABEL
	}
}
