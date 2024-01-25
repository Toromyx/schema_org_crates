/// <https://schema.org/readBy>
pub const READ_BY_PROPERTY_IRI_HTTP: &str = "http://schema.org/readBy";
/// <https://schema.org/readBy>
pub const READ_BY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/readBy";
/// <https://schema.org/readBy>
pub const READ_BY_PROPERTY_LABEL: &str = "readBy";
pub struct ReadByPropertyIri;
impl PartialEq<&str> for ReadByPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == READ_BY_PROPERTY_IRI_HTTP || *other == READ_BY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ReadByPropertyIri> for &str {
	fn eq(&self, other: &ReadByPropertyIri) -> bool {
		*self == READ_BY_PROPERTY_IRI_HTTP || *self == READ_BY_PROPERTY_IRI_HTTPS
	}
}
pub struct ReadByPropertyIriOrLabel;
impl PartialEq<&str> for ReadByPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReadByPropertyIri || *other == READ_BY_PROPERTY_LABEL
	}
}
impl PartialEq<ReadByPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ReadByPropertyIriOrLabel) -> bool {
		*self == ReadByPropertyIri || *self == READ_BY_PROPERTY_LABEL
	}
}
