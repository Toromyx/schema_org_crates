/// <https://schema.org/Beach>
pub const BEACH_IRI_HTTP: &str = "http://schema.org/Beach";
/// <https://schema.org/Beach>
pub const BEACH_IRI_HTTPS: &str = "https://schema.org/Beach";
/// <https://schema.org/Beach>
pub const BEACH_LABEL: &str = "Beach";
pub struct BeachIri;
impl PartialEq<&str> for BeachIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BEACH_IRI_HTTP || *other == BEACH_IRI_HTTPS
	}
}
impl PartialEq<BeachIri> for &str {
	fn eq(&self, other: &BeachIri) -> bool {
		*self == BEACH_IRI_HTTP || *self == BEACH_IRI_HTTPS
	}
}
pub struct BeachIriOrLabel;
impl PartialEq<&str> for BeachIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BeachIri || *other == BEACH_LABEL
	}
}
impl PartialEq<BeachIriOrLabel> for &str {
	fn eq(&self, other: &BeachIriOrLabel) -> bool {
		*self == BeachIri || *self == BEACH_LABEL
	}
}
