/// <https://schema.org/DDxElement>
pub const D_DX_ELEMENT_IRI_HTTP: &str = "http://schema.org/DDxElement";
/// <https://schema.org/DDxElement>
pub const D_DX_ELEMENT_IRI_HTTPS: &str = "https://schema.org/DDxElement";
/// <https://schema.org/DDxElement>
pub const D_DX_ELEMENT_LABEL: &str = "DDxElement";
pub struct DDxElementIri;
impl PartialEq<&str> for DDxElementIri {
	fn eq(&self, other: &&str) -> bool {
		*other == D_DX_ELEMENT_IRI_HTTP || *other == D_DX_ELEMENT_IRI_HTTPS
	}
}
impl PartialEq<DDxElementIri> for &str {
	fn eq(&self, other: &DDxElementIri) -> bool {
		*self == D_DX_ELEMENT_IRI_HTTP || *self == D_DX_ELEMENT_IRI_HTTPS
	}
}
pub struct DDxElementIriOrLabel;
impl PartialEq<&str> for DDxElementIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DDxElementIri || *other == D_DX_ELEMENT_LABEL
	}
}
impl PartialEq<DDxElementIriOrLabel> for &str {
	fn eq(&self, other: &DDxElementIriOrLabel) -> bool {
		*self == DDxElementIri || *self == D_DX_ELEMENT_LABEL
	}
}
