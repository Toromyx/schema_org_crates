/// <https://schema.org/CreativeWork>
pub const CREATIVE_WORK_IRI_HTTP: &str = "http://schema.org/CreativeWork";
/// <https://schema.org/CreativeWork>
pub const CREATIVE_WORK_IRI_HTTPS: &str = "https://schema.org/CreativeWork";
/// <https://schema.org/CreativeWork>
pub const CREATIVE_WORK_LABEL: &str = "CreativeWork";
pub struct CreativeWorkIri;
impl PartialEq<&str> for CreativeWorkIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CREATIVE_WORK_IRI_HTTP || *other == CREATIVE_WORK_IRI_HTTPS
	}
}
impl PartialEq<CreativeWorkIri> for &str {
	fn eq(&self, other: &CreativeWorkIri) -> bool {
		*self == CREATIVE_WORK_IRI_HTTP || *self == CREATIVE_WORK_IRI_HTTPS
	}
}
pub struct CreativeWorkIriOrLabel;
impl PartialEq<&str> for CreativeWorkIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CreativeWorkIri || *other == CREATIVE_WORK_LABEL
	}
}
impl PartialEq<CreativeWorkIriOrLabel> for &str {
	fn eq(&self, other: &CreativeWorkIriOrLabel) -> bool {
		*self == CreativeWorkIri || *self == CREATIVE_WORK_LABEL
	}
}
