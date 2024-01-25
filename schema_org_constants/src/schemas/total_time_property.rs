/// <https://schema.org/totalTime>
pub const TOTAL_TIME_PROPERTY_IRI_HTTP: &str = "http://schema.org/totalTime";
/// <https://schema.org/totalTime>
pub const TOTAL_TIME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/totalTime";
/// <https://schema.org/totalTime>
pub const TOTAL_TIME_PROPERTY_LABEL: &str = "totalTime";
pub struct TotalTimePropertyIri;
impl PartialEq<&str> for TotalTimePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TOTAL_TIME_PROPERTY_IRI_HTTP || *other == TOTAL_TIME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TotalTimePropertyIri> for &str {
	fn eq(&self, other: &TotalTimePropertyIri) -> bool {
		*self == TOTAL_TIME_PROPERTY_IRI_HTTP || *self == TOTAL_TIME_PROPERTY_IRI_HTTPS
	}
}
pub struct TotalTimePropertyIriOrLabel;
impl PartialEq<&str> for TotalTimePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TotalTimePropertyIri || *other == TOTAL_TIME_PROPERTY_LABEL
	}
}
impl PartialEq<TotalTimePropertyIriOrLabel> for &str {
	fn eq(&self, other: &TotalTimePropertyIriOrLabel) -> bool {
		*self == TotalTimePropertyIri || *self == TOTAL_TIME_PROPERTY_LABEL
	}
}
