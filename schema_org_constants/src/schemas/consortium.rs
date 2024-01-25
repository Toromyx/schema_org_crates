/// <https://schema.org/Consortium>
pub const CONSORTIUM_IRI_HTTP: &str = "http://schema.org/Consortium";
/// <https://schema.org/Consortium>
pub const CONSORTIUM_IRI_HTTPS: &str = "https://schema.org/Consortium";
/// <https://schema.org/Consortium>
pub const CONSORTIUM_LABEL: &str = "Consortium";
pub struct ConsortiumIri;
impl PartialEq<&str> for ConsortiumIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CONSORTIUM_IRI_HTTP || *other == CONSORTIUM_IRI_HTTPS
	}
}
impl PartialEq<ConsortiumIri> for &str {
	fn eq(&self, other: &ConsortiumIri) -> bool {
		*self == CONSORTIUM_IRI_HTTP || *self == CONSORTIUM_IRI_HTTPS
	}
}
pub struct ConsortiumIriOrLabel;
impl PartialEq<&str> for ConsortiumIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ConsortiumIri || *other == CONSORTIUM_LABEL
	}
}
impl PartialEq<ConsortiumIriOrLabel> for &str {
	fn eq(&self, other: &ConsortiumIriOrLabel) -> bool {
		*self == ConsortiumIri || *self == CONSORTIUM_LABEL
	}
}
