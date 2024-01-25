/// <https://schema.org/BarOrPub>
pub const BAR_OR_PUB_IRI_HTTP: &str = "http://schema.org/BarOrPub";
/// <https://schema.org/BarOrPub>
pub const BAR_OR_PUB_IRI_HTTPS: &str = "https://schema.org/BarOrPub";
/// <https://schema.org/BarOrPub>
pub const BAR_OR_PUB_LABEL: &str = "BarOrPub";
pub struct BarOrPubIri;
impl PartialEq<&str> for BarOrPubIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BAR_OR_PUB_IRI_HTTP || *other == BAR_OR_PUB_IRI_HTTPS
	}
}
impl PartialEq<BarOrPubIri> for &str {
	fn eq(&self, other: &BarOrPubIri) -> bool {
		*self == BAR_OR_PUB_IRI_HTTP || *self == BAR_OR_PUB_IRI_HTTPS
	}
}
pub struct BarOrPubIriOrLabel;
impl PartialEq<&str> for BarOrPubIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BarOrPubIri || *other == BAR_OR_PUB_LABEL
	}
}
impl PartialEq<BarOrPubIriOrLabel> for &str {
	fn eq(&self, other: &BarOrPubIriOrLabel) -> bool {
		*self == BarOrPubIri || *self == BAR_OR_PUB_LABEL
	}
}
