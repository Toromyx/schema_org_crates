/// <https://schema.org/birthDate>
pub const BIRTH_DATE_PROPERTY_IRI_HTTP: &str = "http://schema.org/birthDate";
/// <https://schema.org/birthDate>
pub const BIRTH_DATE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/birthDate";
/// <https://schema.org/birthDate>
pub const BIRTH_DATE_PROPERTY_LABEL: &str = "birthDate";
pub struct BirthDatePropertyIri;
impl PartialEq<&str> for BirthDatePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BIRTH_DATE_PROPERTY_IRI_HTTP || *other == BIRTH_DATE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BirthDatePropertyIri> for &str {
	fn eq(&self, other: &BirthDatePropertyIri) -> bool {
		*self == BIRTH_DATE_PROPERTY_IRI_HTTP || *self == BIRTH_DATE_PROPERTY_IRI_HTTPS
	}
}
pub struct BirthDatePropertyIriOrLabel;
impl PartialEq<&str> for BirthDatePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BirthDatePropertyIri || *other == BIRTH_DATE_PROPERTY_LABEL
	}
}
impl PartialEq<BirthDatePropertyIriOrLabel> for &str {
	fn eq(&self, other: &BirthDatePropertyIriOrLabel) -> bool {
		*self == BirthDatePropertyIri || *self == BIRTH_DATE_PROPERTY_LABEL
	}
}
