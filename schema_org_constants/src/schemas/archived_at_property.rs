/// <https://schema.org/archivedAt>
pub const ARCHIVED_AT_PROPERTY_IRI_HTTP: &str = "http://schema.org/archivedAt";
/// <https://schema.org/archivedAt>
pub const ARCHIVED_AT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/archivedAt";
/// <https://schema.org/archivedAt>
pub const ARCHIVED_AT_PROPERTY_LABEL: &str = "archivedAt";
pub struct ArchivedAtPropertyIri;
impl PartialEq<&str> for ArchivedAtPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ARCHIVED_AT_PROPERTY_IRI_HTTP || *other == ARCHIVED_AT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ArchivedAtPropertyIri> for &str {
	fn eq(&self, other: &ArchivedAtPropertyIri) -> bool {
		*self == ARCHIVED_AT_PROPERTY_IRI_HTTP || *self == ARCHIVED_AT_PROPERTY_IRI_HTTPS
	}
}
pub struct ArchivedAtPropertyIriOrLabel;
impl PartialEq<&str> for ArchivedAtPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ArchivedAtPropertyIri || *other == ARCHIVED_AT_PROPERTY_LABEL
	}
}
impl PartialEq<ArchivedAtPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ArchivedAtPropertyIriOrLabel) -> bool {
		*self == ArchivedAtPropertyIri || *self == ARCHIVED_AT_PROPERTY_LABEL
	}
}
