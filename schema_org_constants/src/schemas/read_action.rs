/// <https://schema.org/ReadAction>
pub const READ_ACTION_IRI_HTTP: &str = "http://schema.org/ReadAction";
/// <https://schema.org/ReadAction>
pub const READ_ACTION_IRI_HTTPS: &str = "https://schema.org/ReadAction";
/// <https://schema.org/ReadAction>
pub const READ_ACTION_LABEL: &str = "ReadAction";
pub struct ReadActionIri;
impl PartialEq<&str> for ReadActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == READ_ACTION_IRI_HTTP || *other == READ_ACTION_IRI_HTTPS
	}
}
impl PartialEq<ReadActionIri> for &str {
	fn eq(&self, other: &ReadActionIri) -> bool {
		*self == READ_ACTION_IRI_HTTP || *self == READ_ACTION_IRI_HTTPS
	}
}
pub struct ReadActionIriOrLabel;
impl PartialEq<&str> for ReadActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReadActionIri || *other == READ_ACTION_LABEL
	}
}
impl PartialEq<ReadActionIriOrLabel> for &str {
	fn eq(&self, other: &ReadActionIriOrLabel) -> bool {
		*self == ReadActionIri || *self == READ_ACTION_LABEL
	}
}
