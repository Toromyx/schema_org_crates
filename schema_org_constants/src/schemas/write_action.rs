/// <https://schema.org/WriteAction>
pub const WRITE_ACTION_IRI_HTTP: &str = "http://schema.org/WriteAction";
/// <https://schema.org/WriteAction>
pub const WRITE_ACTION_IRI_HTTPS: &str = "https://schema.org/WriteAction";
/// <https://schema.org/WriteAction>
pub const WRITE_ACTION_LABEL: &str = "WriteAction";
pub struct WriteActionIri;
impl PartialEq<&str> for WriteActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WRITE_ACTION_IRI_HTTP || *other == WRITE_ACTION_IRI_HTTPS
	}
}
impl PartialEq<WriteActionIri> for &str {
	fn eq(&self, other: &WriteActionIri) -> bool {
		*self == WRITE_ACTION_IRI_HTTP || *self == WRITE_ACTION_IRI_HTTPS
	}
}
pub struct WriteActionIriOrLabel;
impl PartialEq<&str> for WriteActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WriteActionIri || *other == WRITE_ACTION_LABEL
	}
}
impl PartialEq<WriteActionIriOrLabel> for &str {
	fn eq(&self, other: &WriteActionIriOrLabel) -> bool {
		*self == WriteActionIri || *self == WRITE_ACTION_LABEL
	}
}
