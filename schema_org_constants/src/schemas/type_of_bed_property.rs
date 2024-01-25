/// <https://schema.org/typeOfBed>
pub const TYPE_OF_BED_PROPERTY_IRI_HTTP: &str = "http://schema.org/typeOfBed";
/// <https://schema.org/typeOfBed>
pub const TYPE_OF_BED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/typeOfBed";
/// <https://schema.org/typeOfBed>
pub const TYPE_OF_BED_PROPERTY_LABEL: &str = "typeOfBed";
pub struct TypeOfBedPropertyIri;
impl PartialEq<&str> for TypeOfBedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TYPE_OF_BED_PROPERTY_IRI_HTTP || *other == TYPE_OF_BED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TypeOfBedPropertyIri> for &str {
	fn eq(&self, other: &TypeOfBedPropertyIri) -> bool {
		*self == TYPE_OF_BED_PROPERTY_IRI_HTTP || *self == TYPE_OF_BED_PROPERTY_IRI_HTTPS
	}
}
pub struct TypeOfBedPropertyIriOrLabel;
impl PartialEq<&str> for TypeOfBedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TypeOfBedPropertyIri || *other == TYPE_OF_BED_PROPERTY_LABEL
	}
}
impl PartialEq<TypeOfBedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TypeOfBedPropertyIriOrLabel) -> bool {
		*self == TypeOfBedPropertyIri || *self == TYPE_OF_BED_PROPERTY_LABEL
	}
}
