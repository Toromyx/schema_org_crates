/// <https://schema.org/dateIssued>
pub const DATE_ISSUED_PROPERTY_IRI_HTTP: &str = "http://schema.org/dateIssued";
/// <https://schema.org/dateIssued>
pub const DATE_ISSUED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/dateIssued";
/// <https://schema.org/dateIssued>
pub const DATE_ISSUED_PROPERTY_LABEL: &str = "dateIssued";
pub struct DateIssuedPropertyIri;
impl PartialEq<&str> for DateIssuedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DATE_ISSUED_PROPERTY_IRI_HTTP || *other == DATE_ISSUED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DateIssuedPropertyIri> for &str {
	fn eq(&self, other: &DateIssuedPropertyIri) -> bool {
		*self == DATE_ISSUED_PROPERTY_IRI_HTTP || *self == DATE_ISSUED_PROPERTY_IRI_HTTPS
	}
}
pub struct DateIssuedPropertyIriOrLabel;
impl PartialEq<&str> for DateIssuedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DateIssuedPropertyIri || *other == DATE_ISSUED_PROPERTY_LABEL
	}
}
impl PartialEq<DateIssuedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DateIssuedPropertyIriOrLabel) -> bool {
		*self == DateIssuedPropertyIri || *self == DATE_ISSUED_PROPERTY_LABEL
	}
}
