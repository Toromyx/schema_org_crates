/// <https://schema.org/dateDeleted>
pub const DATE_DELETED_PROPERTY_IRI_HTTP: &str = "http://schema.org/dateDeleted";
/// <https://schema.org/dateDeleted>
pub const DATE_DELETED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/dateDeleted";
/// <https://schema.org/dateDeleted>
pub const DATE_DELETED_PROPERTY_LABEL: &str = "dateDeleted";
pub struct DateDeletedPropertyIri;
impl PartialEq<&str> for DateDeletedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DATE_DELETED_PROPERTY_IRI_HTTP || *other == DATE_DELETED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DateDeletedPropertyIri> for &str {
	fn eq(&self, other: &DateDeletedPropertyIri) -> bool {
		*self == DATE_DELETED_PROPERTY_IRI_HTTP || *self == DATE_DELETED_PROPERTY_IRI_HTTPS
	}
}
pub struct DateDeletedPropertyIriOrLabel;
impl PartialEq<&str> for DateDeletedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DateDeletedPropertyIri || *other == DATE_DELETED_PROPERTY_LABEL
	}
}
impl PartialEq<DateDeletedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DateDeletedPropertyIriOrLabel) -> bool {
		*self == DateDeletedPropertyIri || *self == DATE_DELETED_PROPERTY_LABEL
	}
}
