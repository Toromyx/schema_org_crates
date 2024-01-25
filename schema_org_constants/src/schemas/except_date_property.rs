/// <https://schema.org/exceptDate>
pub const EXCEPT_DATE_PROPERTY_IRI_HTTP: &str = "http://schema.org/exceptDate";
/// <https://schema.org/exceptDate>
pub const EXCEPT_DATE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/exceptDate";
/// <https://schema.org/exceptDate>
pub const EXCEPT_DATE_PROPERTY_LABEL: &str = "exceptDate";
pub struct ExceptDatePropertyIri;
impl PartialEq<&str> for ExceptDatePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EXCEPT_DATE_PROPERTY_IRI_HTTP || *other == EXCEPT_DATE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ExceptDatePropertyIri> for &str {
	fn eq(&self, other: &ExceptDatePropertyIri) -> bool {
		*self == EXCEPT_DATE_PROPERTY_IRI_HTTP || *self == EXCEPT_DATE_PROPERTY_IRI_HTTPS
	}
}
pub struct ExceptDatePropertyIriOrLabel;
impl PartialEq<&str> for ExceptDatePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ExceptDatePropertyIri || *other == EXCEPT_DATE_PROPERTY_LABEL
	}
}
impl PartialEq<ExceptDatePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ExceptDatePropertyIriOrLabel) -> bool {
		*self == ExceptDatePropertyIri || *self == EXCEPT_DATE_PROPERTY_LABEL
	}
}
