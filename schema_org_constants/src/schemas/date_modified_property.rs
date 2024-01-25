/// <https://schema.org/dateModified>
pub const DATE_MODIFIED_PROPERTY_IRI_HTTP: &str = "http://schema.org/dateModified";
/// <https://schema.org/dateModified>
pub const DATE_MODIFIED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/dateModified";
/// <https://schema.org/dateModified>
pub const DATE_MODIFIED_PROPERTY_LABEL: &str = "dateModified";
pub struct DateModifiedPropertyIri;
impl PartialEq<&str> for DateModifiedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DATE_MODIFIED_PROPERTY_IRI_HTTP || *other == DATE_MODIFIED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DateModifiedPropertyIri> for &str {
	fn eq(&self, other: &DateModifiedPropertyIri) -> bool {
		*self == DATE_MODIFIED_PROPERTY_IRI_HTTP || *self == DATE_MODIFIED_PROPERTY_IRI_HTTPS
	}
}
pub struct DateModifiedPropertyIriOrLabel;
impl PartialEq<&str> for DateModifiedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DateModifiedPropertyIri || *other == DATE_MODIFIED_PROPERTY_LABEL
	}
}
impl PartialEq<DateModifiedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DateModifiedPropertyIriOrLabel) -> bool {
		*self == DateModifiedPropertyIri || *self == DATE_MODIFIED_PROPERTY_LABEL
	}
}
