/// <https://schema.org/cutoffTime>
pub const CUTOFF_TIME_PROPERTY_IRI_HTTP: &str = "http://schema.org/cutoffTime";
/// <https://schema.org/cutoffTime>
pub const CUTOFF_TIME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/cutoffTime";
/// <https://schema.org/cutoffTime>
pub const CUTOFF_TIME_PROPERTY_LABEL: &str = "cutoffTime";
pub struct CutoffTimePropertyIri;
impl PartialEq<&str> for CutoffTimePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CUTOFF_TIME_PROPERTY_IRI_HTTP || *other == CUTOFF_TIME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CutoffTimePropertyIri> for &str {
	fn eq(&self, other: &CutoffTimePropertyIri) -> bool {
		*self == CUTOFF_TIME_PROPERTY_IRI_HTTP || *self == CUTOFF_TIME_PROPERTY_IRI_HTTPS
	}
}
pub struct CutoffTimePropertyIriOrLabel;
impl PartialEq<&str> for CutoffTimePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CutoffTimePropertyIri || *other == CUTOFF_TIME_PROPERTY_LABEL
	}
}
impl PartialEq<CutoffTimePropertyIriOrLabel> for &str {
	fn eq(&self, other: &CutoffTimePropertyIriOrLabel) -> bool {
		*self == CutoffTimePropertyIri || *self == CUTOFF_TIME_PROPERTY_LABEL
	}
}
