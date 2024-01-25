/// <https://schema.org/Saturday>
pub const SATURDAY_IRI_HTTP: &str = "http://schema.org/Saturday";
/// <https://schema.org/Saturday>
pub const SATURDAY_IRI_HTTPS: &str = "https://schema.org/Saturday";
/// <https://schema.org/Saturday>
pub const SATURDAY_LABEL: &str = "Saturday";
pub struct SaturdayIri;
impl PartialEq<&str> for SaturdayIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SATURDAY_IRI_HTTP || *other == SATURDAY_IRI_HTTPS
	}
}
impl PartialEq<SaturdayIri> for &str {
	fn eq(&self, other: &SaturdayIri) -> bool {
		*self == SATURDAY_IRI_HTTP || *self == SATURDAY_IRI_HTTPS
	}
}
pub struct SaturdayIriOrLabel;
impl PartialEq<&str> for SaturdayIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SaturdayIri || *other == SATURDAY_LABEL
	}
}
impl PartialEq<SaturdayIriOrLabel> for &str {
	fn eq(&self, other: &SaturdayIriOrLabel) -> bool {
		*self == SaturdayIri || *self == SATURDAY_LABEL
	}
}
