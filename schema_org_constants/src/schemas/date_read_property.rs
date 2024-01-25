/// <https://schema.org/dateRead>
pub const DATE_READ_PROPERTY_IRI_HTTP: &str = "http://schema.org/dateRead";
/// <https://schema.org/dateRead>
pub const DATE_READ_PROPERTY_IRI_HTTPS: &str = "https://schema.org/dateRead";
/// <https://schema.org/dateRead>
pub const DATE_READ_PROPERTY_LABEL: &str = "dateRead";
pub struct DateReadPropertyIri;
impl PartialEq<&str> for DateReadPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DATE_READ_PROPERTY_IRI_HTTP || *other == DATE_READ_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DateReadPropertyIri> for &str {
	fn eq(&self, other: &DateReadPropertyIri) -> bool {
		*self == DATE_READ_PROPERTY_IRI_HTTP || *self == DATE_READ_PROPERTY_IRI_HTTPS
	}
}
pub struct DateReadPropertyIriOrLabel;
impl PartialEq<&str> for DateReadPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DateReadPropertyIri || *other == DATE_READ_PROPERTY_LABEL
	}
}
impl PartialEq<DateReadPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DateReadPropertyIriOrLabel) -> bool {
		*self == DateReadPropertyIri || *self == DATE_READ_PROPERTY_LABEL
	}
}
