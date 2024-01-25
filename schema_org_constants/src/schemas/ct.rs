/// <https://schema.org/CT>
pub const CT_IRI_HTTP: &str = "http://schema.org/CT";
/// <https://schema.org/CT>
pub const CT_IRI_HTTPS: &str = "https://schema.org/CT";
/// <https://schema.org/CT>
pub const CT_LABEL: &str = "CT";
pub struct CtIri;
impl PartialEq<&str> for CtIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CT_IRI_HTTP || *other == CT_IRI_HTTPS
	}
}
impl PartialEq<CtIri> for &str {
	fn eq(&self, other: &CtIri) -> bool {
		*self == CT_IRI_HTTP || *self == CT_IRI_HTTPS
	}
}
pub struct CtIriOrLabel;
impl PartialEq<&str> for CtIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CtIri || *other == CT_LABEL
	}
}
impl PartialEq<CtIriOrLabel> for &str {
	fn eq(&self, other: &CtIriOrLabel) -> bool {
		*self == CtIri || *self == CT_LABEL
	}
}
