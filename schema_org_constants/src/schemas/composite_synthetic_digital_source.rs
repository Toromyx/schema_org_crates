/// <https://schema.org/CompositeSyntheticDigitalSource>
pub const COMPOSITE_SYNTHETIC_DIGITAL_SOURCE_IRI_HTTP: &str =
	"http://schema.org/CompositeSyntheticDigitalSource";
/// <https://schema.org/CompositeSyntheticDigitalSource>
pub const COMPOSITE_SYNTHETIC_DIGITAL_SOURCE_IRI_HTTPS: &str =
	"https://schema.org/CompositeSyntheticDigitalSource";
/// <https://schema.org/CompositeSyntheticDigitalSource>
pub const COMPOSITE_SYNTHETIC_DIGITAL_SOURCE_LABEL: &str = "CompositeSyntheticDigitalSource";
pub struct CompositeSyntheticDigitalSourceIri;
impl PartialEq<&str> for CompositeSyntheticDigitalSourceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COMPOSITE_SYNTHETIC_DIGITAL_SOURCE_IRI_HTTP
			|| *other == COMPOSITE_SYNTHETIC_DIGITAL_SOURCE_IRI_HTTPS
	}
}
impl PartialEq<CompositeSyntheticDigitalSourceIri> for &str {
	fn eq(&self, other: &CompositeSyntheticDigitalSourceIri) -> bool {
		*self == COMPOSITE_SYNTHETIC_DIGITAL_SOURCE_IRI_HTTP
			|| *self == COMPOSITE_SYNTHETIC_DIGITAL_SOURCE_IRI_HTTPS
	}
}
pub struct CompositeSyntheticDigitalSourceIriOrLabel;
impl PartialEq<&str> for CompositeSyntheticDigitalSourceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CompositeSyntheticDigitalSourceIri
			|| *other == COMPOSITE_SYNTHETIC_DIGITAL_SOURCE_LABEL
	}
}
impl PartialEq<CompositeSyntheticDigitalSourceIriOrLabel> for &str {
	fn eq(&self, other: &CompositeSyntheticDigitalSourceIriOrLabel) -> bool {
		*self == CompositeSyntheticDigitalSourceIri
			|| *self == COMPOSITE_SYNTHETIC_DIGITAL_SOURCE_LABEL
	}
}
