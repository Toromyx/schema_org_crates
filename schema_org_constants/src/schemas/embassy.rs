/// <https://schema.org/Embassy>
pub const EMBASSY_IRI_HTTP: &str = "http://schema.org/Embassy";
/// <https://schema.org/Embassy>
pub const EMBASSY_IRI_HTTPS: &str = "https://schema.org/Embassy";
/// <https://schema.org/Embassy>
pub const EMBASSY_LABEL: &str = "Embassy";
pub struct EmbassyIri;
impl PartialEq<&str> for EmbassyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EMBASSY_IRI_HTTP || *other == EMBASSY_IRI_HTTPS
	}
}
impl PartialEq<EmbassyIri> for &str {
	fn eq(&self, other: &EmbassyIri) -> bool {
		*self == EMBASSY_IRI_HTTP || *self == EMBASSY_IRI_HTTPS
	}
}
pub struct EmbassyIriOrLabel;
impl PartialEq<&str> for EmbassyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EmbassyIri || *other == EMBASSY_LABEL
	}
}
impl PartialEq<EmbassyIriOrLabel> for &str {
	fn eq(&self, other: &EmbassyIriOrLabel) -> bool {
		*self == EmbassyIri || *self == EMBASSY_LABEL
	}
}
