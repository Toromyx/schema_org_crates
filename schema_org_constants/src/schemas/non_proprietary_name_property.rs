/// <https://schema.org/nonProprietaryName>
pub const NON_PROPRIETARY_NAME_PROPERTY_IRI_HTTP: &str = "http://schema.org/nonProprietaryName";
/// <https://schema.org/nonProprietaryName>
pub const NON_PROPRIETARY_NAME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/nonProprietaryName";
/// <https://schema.org/nonProprietaryName>
pub const NON_PROPRIETARY_NAME_PROPERTY_LABEL: &str = "nonProprietaryName";
pub struct NonProprietaryNamePropertyIri;
impl PartialEq<&str> for NonProprietaryNamePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NON_PROPRIETARY_NAME_PROPERTY_IRI_HTTP
			|| *other == NON_PROPRIETARY_NAME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NonProprietaryNamePropertyIri> for &str {
	fn eq(&self, other: &NonProprietaryNamePropertyIri) -> bool {
		*self == NON_PROPRIETARY_NAME_PROPERTY_IRI_HTTP
			|| *self == NON_PROPRIETARY_NAME_PROPERTY_IRI_HTTPS
	}
}
pub struct NonProprietaryNamePropertyIriOrLabel;
impl PartialEq<&str> for NonProprietaryNamePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NonProprietaryNamePropertyIri || *other == NON_PROPRIETARY_NAME_PROPERTY_LABEL
	}
}
impl PartialEq<NonProprietaryNamePropertyIriOrLabel> for &str {
	fn eq(&self, other: &NonProprietaryNamePropertyIriOrLabel) -> bool {
		*self == NonProprietaryNamePropertyIri || *self == NON_PROPRIETARY_NAME_PROPERTY_LABEL
	}
}
