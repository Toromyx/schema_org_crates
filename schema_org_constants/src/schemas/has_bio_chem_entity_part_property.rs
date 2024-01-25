/// <https://schema.org/hasBioChemEntityPart>
pub const HAS_BIO_CHEM_ENTITY_PART_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/hasBioChemEntityPart";
/// <https://schema.org/hasBioChemEntityPart>
pub const HAS_BIO_CHEM_ENTITY_PART_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/hasBioChemEntityPart";
/// <https://schema.org/hasBioChemEntityPart>
pub const HAS_BIO_CHEM_ENTITY_PART_PROPERTY_LABEL: &str = "hasBioChemEntityPart";
pub struct HasBioChemEntityPartPropertyIri;
impl PartialEq<&str> for HasBioChemEntityPartPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HAS_BIO_CHEM_ENTITY_PART_PROPERTY_IRI_HTTP
			|| *other == HAS_BIO_CHEM_ENTITY_PART_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HasBioChemEntityPartPropertyIri> for &str {
	fn eq(&self, other: &HasBioChemEntityPartPropertyIri) -> bool {
		*self == HAS_BIO_CHEM_ENTITY_PART_PROPERTY_IRI_HTTP
			|| *self == HAS_BIO_CHEM_ENTITY_PART_PROPERTY_IRI_HTTPS
	}
}
pub struct HasBioChemEntityPartPropertyIriOrLabel;
impl PartialEq<&str> for HasBioChemEntityPartPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HasBioChemEntityPartPropertyIri
			|| *other == HAS_BIO_CHEM_ENTITY_PART_PROPERTY_LABEL
	}
}
impl PartialEq<HasBioChemEntityPartPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HasBioChemEntityPartPropertyIriOrLabel) -> bool {
		*self == HasBioChemEntityPartPropertyIri || *self == HAS_BIO_CHEM_ENTITY_PART_PROPERTY_LABEL
	}
}
