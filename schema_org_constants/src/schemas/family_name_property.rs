/// <https://schema.org/familyName>
pub const FAMILY_NAME_PROPERTY_IRI_HTTP: &str = "http://schema.org/familyName";
/// <https://schema.org/familyName>
pub const FAMILY_NAME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/familyName";
/// <https://schema.org/familyName>
pub const FAMILY_NAME_PROPERTY_LABEL: &str = "familyName";
pub struct FamilyNamePropertyIri;
impl PartialEq<&str> for FamilyNamePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FAMILY_NAME_PROPERTY_IRI_HTTP || *other == FAMILY_NAME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<FamilyNamePropertyIri> for &str {
	fn eq(&self, other: &FamilyNamePropertyIri) -> bool {
		*self == FAMILY_NAME_PROPERTY_IRI_HTTP || *self == FAMILY_NAME_PROPERTY_IRI_HTTPS
	}
}
pub struct FamilyNamePropertyIriOrLabel;
impl PartialEq<&str> for FamilyNamePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FamilyNamePropertyIri || *other == FAMILY_NAME_PROPERTY_LABEL
	}
}
impl PartialEq<FamilyNamePropertyIriOrLabel> for &str {
	fn eq(&self, other: &FamilyNamePropertyIriOrLabel) -> bool {
		*self == FamilyNamePropertyIri || *self == FAMILY_NAME_PROPERTY_LABEL
	}
}
