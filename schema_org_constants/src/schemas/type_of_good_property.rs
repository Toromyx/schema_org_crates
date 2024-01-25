/// <https://schema.org/typeOfGood>
pub const TYPE_OF_GOOD_PROPERTY_IRI_HTTP: &str = "http://schema.org/typeOfGood";
/// <https://schema.org/typeOfGood>
pub const TYPE_OF_GOOD_PROPERTY_IRI_HTTPS: &str = "https://schema.org/typeOfGood";
/// <https://schema.org/typeOfGood>
pub const TYPE_OF_GOOD_PROPERTY_LABEL: &str = "typeOfGood";
pub struct TypeOfGoodPropertyIri;
impl PartialEq<&str> for TypeOfGoodPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TYPE_OF_GOOD_PROPERTY_IRI_HTTP || *other == TYPE_OF_GOOD_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TypeOfGoodPropertyIri> for &str {
	fn eq(&self, other: &TypeOfGoodPropertyIri) -> bool {
		*self == TYPE_OF_GOOD_PROPERTY_IRI_HTTP || *self == TYPE_OF_GOOD_PROPERTY_IRI_HTTPS
	}
}
pub struct TypeOfGoodPropertyIriOrLabel;
impl PartialEq<&str> for TypeOfGoodPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TypeOfGoodPropertyIri || *other == TYPE_OF_GOOD_PROPERTY_LABEL
	}
}
impl PartialEq<TypeOfGoodPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TypeOfGoodPropertyIriOrLabel) -> bool {
		*self == TypeOfGoodPropertyIri || *self == TYPE_OF_GOOD_PROPERTY_LABEL
	}
}
