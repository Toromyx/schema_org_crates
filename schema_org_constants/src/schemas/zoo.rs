/// <https://schema.org/Zoo>
pub const ZOO_IRI_HTTP: &str = "http://schema.org/Zoo";
/// <https://schema.org/Zoo>
pub const ZOO_IRI_HTTPS: &str = "https://schema.org/Zoo";
/// <https://schema.org/Zoo>
pub const ZOO_LABEL: &str = "Zoo";
pub struct ZooIri;
impl PartialEq<&str> for ZooIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ZOO_IRI_HTTP || *other == ZOO_IRI_HTTPS
	}
}
impl PartialEq<ZooIri> for &str {
	fn eq(&self, other: &ZooIri) -> bool {
		*self == ZOO_IRI_HTTP || *self == ZOO_IRI_HTTPS
	}
}
pub struct ZooIriOrLabel;
impl PartialEq<&str> for ZooIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ZooIri || *other == ZOO_LABEL
	}
}
impl PartialEq<ZooIriOrLabel> for &str {
	fn eq(&self, other: &ZooIriOrLabel) -> bool {
		*self == ZooIri || *self == ZOO_LABEL
	}
}
