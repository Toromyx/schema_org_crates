/// <https://schema.org/isPartOfBioChemEntity>
pub const IS_PART_OF_BIO_CHEM_ENTITY_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/isPartOfBioChemEntity";
/// <https://schema.org/isPartOfBioChemEntity>
pub const IS_PART_OF_BIO_CHEM_ENTITY_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/isPartOfBioChemEntity";
/// <https://schema.org/isPartOfBioChemEntity>
pub const IS_PART_OF_BIO_CHEM_ENTITY_PROPERTY_LABEL: &str = "isPartOfBioChemEntity";
pub struct IsPartOfBioChemEntityPropertyIri;
impl PartialEq<&str> for IsPartOfBioChemEntityPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IS_PART_OF_BIO_CHEM_ENTITY_PROPERTY_IRI_HTTP
			|| *other == IS_PART_OF_BIO_CHEM_ENTITY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IsPartOfBioChemEntityPropertyIri> for &str {
	fn eq(&self, other: &IsPartOfBioChemEntityPropertyIri) -> bool {
		*self == IS_PART_OF_BIO_CHEM_ENTITY_PROPERTY_IRI_HTTP
			|| *self == IS_PART_OF_BIO_CHEM_ENTITY_PROPERTY_IRI_HTTPS
	}
}
pub struct IsPartOfBioChemEntityPropertyIriOrLabel;
impl PartialEq<&str> for IsPartOfBioChemEntityPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IsPartOfBioChemEntityPropertyIri
			|| *other == IS_PART_OF_BIO_CHEM_ENTITY_PROPERTY_LABEL
	}
}
impl PartialEq<IsPartOfBioChemEntityPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IsPartOfBioChemEntityPropertyIriOrLabel) -> bool {
		*self == IsPartOfBioChemEntityPropertyIri
			|| *self == IS_PART_OF_BIO_CHEM_ENTITY_PROPERTY_LABEL
	}
}
