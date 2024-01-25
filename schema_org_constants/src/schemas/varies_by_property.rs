/// <https://schema.org/variesBy>
pub const VARIES_BY_PROPERTY_IRI_HTTP: &str = "http://schema.org/variesBy";
/// <https://schema.org/variesBy>
pub const VARIES_BY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/variesBy";
/// <https://schema.org/variesBy>
pub const VARIES_BY_PROPERTY_LABEL: &str = "variesBy";
pub struct VariesByPropertyIri;
impl PartialEq<&str> for VariesByPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VARIES_BY_PROPERTY_IRI_HTTP || *other == VARIES_BY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<VariesByPropertyIri> for &str {
	fn eq(&self, other: &VariesByPropertyIri) -> bool {
		*self == VARIES_BY_PROPERTY_IRI_HTTP || *self == VARIES_BY_PROPERTY_IRI_HTTPS
	}
}
pub struct VariesByPropertyIriOrLabel;
impl PartialEq<&str> for VariesByPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VariesByPropertyIri || *other == VARIES_BY_PROPERTY_LABEL
	}
}
impl PartialEq<VariesByPropertyIriOrLabel> for &str {
	fn eq(&self, other: &VariesByPropertyIriOrLabel) -> bool {
		*self == VariesByPropertyIri || *self == VARIES_BY_PROPERTY_LABEL
	}
}
