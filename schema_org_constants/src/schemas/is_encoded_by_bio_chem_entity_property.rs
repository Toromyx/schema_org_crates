/// <https://schema.org/isEncodedByBioChemEntity>
pub const IS_ENCODED_BY_BIO_CHEM_ENTITY_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/isEncodedByBioChemEntity";
/// <https://schema.org/isEncodedByBioChemEntity>
pub const IS_ENCODED_BY_BIO_CHEM_ENTITY_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/isEncodedByBioChemEntity";
/// <https://schema.org/isEncodedByBioChemEntity>
pub const IS_ENCODED_BY_BIO_CHEM_ENTITY_PROPERTY_LABEL: &str = "isEncodedByBioChemEntity";
pub struct IsEncodedByBioChemEntityPropertyIri;
impl PartialEq<&str> for IsEncodedByBioChemEntityPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IS_ENCODED_BY_BIO_CHEM_ENTITY_PROPERTY_IRI_HTTP
			|| *other == IS_ENCODED_BY_BIO_CHEM_ENTITY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IsEncodedByBioChemEntityPropertyIri> for &str {
	fn eq(&self, other: &IsEncodedByBioChemEntityPropertyIri) -> bool {
		*self == IS_ENCODED_BY_BIO_CHEM_ENTITY_PROPERTY_IRI_HTTP
			|| *self == IS_ENCODED_BY_BIO_CHEM_ENTITY_PROPERTY_IRI_HTTPS
	}
}
pub struct IsEncodedByBioChemEntityPropertyIriOrLabel;
impl PartialEq<&str> for IsEncodedByBioChemEntityPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IsEncodedByBioChemEntityPropertyIri
			|| *other == IS_ENCODED_BY_BIO_CHEM_ENTITY_PROPERTY_LABEL
	}
}
impl PartialEq<IsEncodedByBioChemEntityPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IsEncodedByBioChemEntityPropertyIriOrLabel) -> bool {
		*self == IsEncodedByBioChemEntityPropertyIri
			|| *self == IS_ENCODED_BY_BIO_CHEM_ENTITY_PROPERTY_LABEL
	}
}
