/// <https://schema.org/depth>
pub const DEPTH_PROPERTY_IRI_HTTP: &str = "http://schema.org/depth";
/// <https://schema.org/depth>
pub const DEPTH_PROPERTY_IRI_HTTPS: &str = "https://schema.org/depth";
/// <https://schema.org/depth>
pub const DEPTH_PROPERTY_LABEL: &str = "depth";
pub struct DepthPropertyIri;
impl PartialEq<&str> for DepthPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DEPTH_PROPERTY_IRI_HTTP || *other == DEPTH_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DepthPropertyIri> for &str {
	fn eq(&self, other: &DepthPropertyIri) -> bool {
		*self == DEPTH_PROPERTY_IRI_HTTP || *self == DEPTH_PROPERTY_IRI_HTTPS
	}
}
pub struct DepthPropertyIriOrLabel;
impl PartialEq<&str> for DepthPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DepthPropertyIri || *other == DEPTH_PROPERTY_LABEL
	}
}
impl PartialEq<DepthPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DepthPropertyIriOrLabel) -> bool {
		*self == DepthPropertyIri || *self == DEPTH_PROPERTY_LABEL
	}
}
