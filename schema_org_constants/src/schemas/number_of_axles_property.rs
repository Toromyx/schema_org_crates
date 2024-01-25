/// <https://schema.org/numberOfAxles>
pub const NUMBER_OF_AXLES_PROPERTY_IRI_HTTP: &str = "http://schema.org/numberOfAxles";
/// <https://schema.org/numberOfAxles>
pub const NUMBER_OF_AXLES_PROPERTY_IRI_HTTPS: &str = "https://schema.org/numberOfAxles";
/// <https://schema.org/numberOfAxles>
pub const NUMBER_OF_AXLES_PROPERTY_LABEL: &str = "numberOfAxles";
pub struct NumberOfAxlesPropertyIri;
impl PartialEq<&str> for NumberOfAxlesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NUMBER_OF_AXLES_PROPERTY_IRI_HTTP || *other == NUMBER_OF_AXLES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NumberOfAxlesPropertyIri> for &str {
	fn eq(&self, other: &NumberOfAxlesPropertyIri) -> bool {
		*self == NUMBER_OF_AXLES_PROPERTY_IRI_HTTP || *self == NUMBER_OF_AXLES_PROPERTY_IRI_HTTPS
	}
}
pub struct NumberOfAxlesPropertyIriOrLabel;
impl PartialEq<&str> for NumberOfAxlesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NumberOfAxlesPropertyIri || *other == NUMBER_OF_AXLES_PROPERTY_LABEL
	}
}
impl PartialEq<NumberOfAxlesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NumberOfAxlesPropertyIriOrLabel) -> bool {
		*self == NumberOfAxlesPropertyIri || *self == NUMBER_OF_AXLES_PROPERTY_LABEL
	}
}
