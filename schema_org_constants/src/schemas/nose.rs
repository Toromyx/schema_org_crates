/// <https://schema.org/Nose>
pub const NOSE_IRI_HTTP: &str = "http://schema.org/Nose";
/// <https://schema.org/Nose>
pub const NOSE_IRI_HTTPS: &str = "https://schema.org/Nose";
/// <https://schema.org/Nose>
pub const NOSE_LABEL: &str = "Nose";
pub struct NoseIri;
impl PartialEq<&str> for NoseIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NOSE_IRI_HTTP || *other == NOSE_IRI_HTTPS
	}
}
impl PartialEq<NoseIri> for &str {
	fn eq(&self, other: &NoseIri) -> bool {
		*self == NOSE_IRI_HTTP || *self == NOSE_IRI_HTTPS
	}
}
pub struct NoseIriOrLabel;
impl PartialEq<&str> for NoseIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NoseIri || *other == NOSE_LABEL
	}
}
impl PartialEq<NoseIriOrLabel> for &str {
	fn eq(&self, other: &NoseIriOrLabel) -> bool {
		*self == NoseIri || *self == NOSE_LABEL
	}
}
