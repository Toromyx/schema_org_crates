/// <https://schema.org/dateCreated>
pub const DATE_CREATED_PROPERTY_IRI_HTTP: &str = "http://schema.org/dateCreated";
/// <https://schema.org/dateCreated>
pub const DATE_CREATED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/dateCreated";
/// <https://schema.org/dateCreated>
pub const DATE_CREATED_PROPERTY_LABEL: &str = "dateCreated";
pub struct DateCreatedPropertyIri;
impl PartialEq<&str> for DateCreatedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DATE_CREATED_PROPERTY_IRI_HTTP || *other == DATE_CREATED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DateCreatedPropertyIri> for &str {
	fn eq(&self, other: &DateCreatedPropertyIri) -> bool {
		*self == DATE_CREATED_PROPERTY_IRI_HTTP || *self == DATE_CREATED_PROPERTY_IRI_HTTPS
	}
}
pub struct DateCreatedPropertyIriOrLabel;
impl PartialEq<&str> for DateCreatedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DateCreatedPropertyIri || *other == DATE_CREATED_PROPERTY_LABEL
	}
}
impl PartialEq<DateCreatedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DateCreatedPropertyIriOrLabel) -> bool {
		*self == DateCreatedPropertyIri || *self == DATE_CREATED_PROPERTY_LABEL
	}
}
