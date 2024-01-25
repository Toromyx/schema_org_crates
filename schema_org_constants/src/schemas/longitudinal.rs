/// <https://schema.org/Longitudinal>
pub const LONGITUDINAL_IRI_HTTP: &str = "http://schema.org/Longitudinal";
/// <https://schema.org/Longitudinal>
pub const LONGITUDINAL_IRI_HTTPS: &str = "https://schema.org/Longitudinal";
/// <https://schema.org/Longitudinal>
pub const LONGITUDINAL_LABEL: &str = "Longitudinal";
pub struct LongitudinalIri;
impl PartialEq<&str> for LongitudinalIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LONGITUDINAL_IRI_HTTP || *other == LONGITUDINAL_IRI_HTTPS
	}
}
impl PartialEq<LongitudinalIri> for &str {
	fn eq(&self, other: &LongitudinalIri) -> bool {
		*self == LONGITUDINAL_IRI_HTTP || *self == LONGITUDINAL_IRI_HTTPS
	}
}
pub struct LongitudinalIriOrLabel;
impl PartialEq<&str> for LongitudinalIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LongitudinalIri || *other == LONGITUDINAL_LABEL
	}
}
impl PartialEq<LongitudinalIriOrLabel> for &str {
	fn eq(&self, other: &LongitudinalIriOrLabel) -> bool {
		*self == LongitudinalIri || *self == LONGITUDINAL_LABEL
	}
}
