/// <https://schema.org/Geriatric>
pub const GERIATRIC_IRI_HTTP: &str = "http://schema.org/Geriatric";
/// <https://schema.org/Geriatric>
pub const GERIATRIC_IRI_HTTPS: &str = "https://schema.org/Geriatric";
/// <https://schema.org/Geriatric>
pub const GERIATRIC_LABEL: &str = "Geriatric";
pub struct GeriatricIri;
impl PartialEq<&str> for GeriatricIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GERIATRIC_IRI_HTTP || *other == GERIATRIC_IRI_HTTPS
	}
}
impl PartialEq<GeriatricIri> for &str {
	fn eq(&self, other: &GeriatricIri) -> bool {
		*self == GERIATRIC_IRI_HTTP || *self == GERIATRIC_IRI_HTTPS
	}
}
pub struct GeriatricIriOrLabel;
impl PartialEq<&str> for GeriatricIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GeriatricIri || *other == GERIATRIC_LABEL
	}
}
impl PartialEq<GeriatricIriOrLabel> for &str {
	fn eq(&self, other: &GeriatricIriOrLabel) -> bool {
		*self == GeriatricIri || *self == GERIATRIC_LABEL
	}
}
