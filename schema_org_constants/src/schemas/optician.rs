/// <https://schema.org/Optician>
pub const OPTICIAN_IRI_HTTP: &str = "http://schema.org/Optician";
/// <https://schema.org/Optician>
pub const OPTICIAN_IRI_HTTPS: &str = "https://schema.org/Optician";
/// <https://schema.org/Optician>
pub const OPTICIAN_LABEL: &str = "Optician";
pub struct OpticianIri;
impl PartialEq<&str> for OpticianIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OPTICIAN_IRI_HTTP || *other == OPTICIAN_IRI_HTTPS
	}
}
impl PartialEq<OpticianIri> for &str {
	fn eq(&self, other: &OpticianIri) -> bool {
		*self == OPTICIAN_IRI_HTTP || *self == OPTICIAN_IRI_HTTPS
	}
}
pub struct OpticianIriOrLabel;
impl PartialEq<&str> for OpticianIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OpticianIri || *other == OPTICIAN_LABEL
	}
}
impl PartialEq<OpticianIriOrLabel> for &str {
	fn eq(&self, other: &OpticianIriOrLabel) -> bool {
		*self == OpticianIri || *self == OPTICIAN_LABEL
	}
}
