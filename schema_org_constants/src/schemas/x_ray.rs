/// <https://schema.org/XRay>
pub const X_RAY_IRI_HTTP: &str = "http://schema.org/XRay";
/// <https://schema.org/XRay>
pub const X_RAY_IRI_HTTPS: &str = "https://schema.org/XRay";
/// <https://schema.org/XRay>
pub const X_RAY_LABEL: &str = "XRay";
pub struct XRayIri;
impl PartialEq<&str> for XRayIri {
	fn eq(&self, other: &&str) -> bool {
		*other == X_RAY_IRI_HTTP || *other == X_RAY_IRI_HTTPS
	}
}
impl PartialEq<XRayIri> for &str {
	fn eq(&self, other: &XRayIri) -> bool {
		*self == X_RAY_IRI_HTTP || *self == X_RAY_IRI_HTTPS
	}
}
pub struct XRayIriOrLabel;
impl PartialEq<&str> for XRayIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == XRayIri || *other == X_RAY_LABEL
	}
}
impl PartialEq<XRayIriOrLabel> for &str {
	fn eq(&self, other: &XRayIriOrLabel) -> bool {
		*self == XRayIri || *self == X_RAY_LABEL
	}
}
