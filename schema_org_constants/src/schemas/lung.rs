/// <https://schema.org/Lung>
pub const LUNG_IRI_HTTP: &str = "http://schema.org/Lung";
/// <https://schema.org/Lung>
pub const LUNG_IRI_HTTPS: &str = "https://schema.org/Lung";
/// <https://schema.org/Lung>
pub const LUNG_LABEL: &str = "Lung";
pub struct LungIri;
impl PartialEq<&str> for LungIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LUNG_IRI_HTTP || *other == LUNG_IRI_HTTPS
	}
}
impl PartialEq<LungIri> for &str {
	fn eq(&self, other: &LungIri) -> bool {
		*self == LUNG_IRI_HTTP || *self == LUNG_IRI_HTTPS
	}
}
pub struct LungIriOrLabel;
impl PartialEq<&str> for LungIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LungIri || *other == LUNG_LABEL
	}
}
impl PartialEq<LungIriOrLabel> for &str {
	fn eq(&self, other: &LungIriOrLabel) -> bool {
		*self == LungIri || *self == LUNG_LABEL
	}
}
