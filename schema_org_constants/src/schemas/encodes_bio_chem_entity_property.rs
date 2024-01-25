/// <https://schema.org/encodesBioChemEntity>
pub const ENCODES_BIO_CHEM_ENTITY_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/encodesBioChemEntity";
/// <https://schema.org/encodesBioChemEntity>
pub const ENCODES_BIO_CHEM_ENTITY_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/encodesBioChemEntity";
/// <https://schema.org/encodesBioChemEntity>
pub const ENCODES_BIO_CHEM_ENTITY_PROPERTY_LABEL: &str = "encodesBioChemEntity";
pub struct EncodesBioChemEntityPropertyIri;
impl PartialEq<&str> for EncodesBioChemEntityPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ENCODES_BIO_CHEM_ENTITY_PROPERTY_IRI_HTTP
			|| *other == ENCODES_BIO_CHEM_ENTITY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EncodesBioChemEntityPropertyIri> for &str {
	fn eq(&self, other: &EncodesBioChemEntityPropertyIri) -> bool {
		*self == ENCODES_BIO_CHEM_ENTITY_PROPERTY_IRI_HTTP
			|| *self == ENCODES_BIO_CHEM_ENTITY_PROPERTY_IRI_HTTPS
	}
}
pub struct EncodesBioChemEntityPropertyIriOrLabel;
impl PartialEq<&str> for EncodesBioChemEntityPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EncodesBioChemEntityPropertyIri
			|| *other == ENCODES_BIO_CHEM_ENTITY_PROPERTY_LABEL
	}
}
impl PartialEq<EncodesBioChemEntityPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EncodesBioChemEntityPropertyIriOrLabel) -> bool {
		*self == EncodesBioChemEntityPropertyIri || *self == ENCODES_BIO_CHEM_ENTITY_PROPERTY_LABEL
	}
}
