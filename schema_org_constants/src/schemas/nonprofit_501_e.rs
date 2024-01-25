/// <https://schema.org/Nonprofit501e>
pub const NONPROFIT_501_E_IRI_HTTP: &str = "http://schema.org/Nonprofit501e";
/// <https://schema.org/Nonprofit501e>
pub const NONPROFIT_501_E_IRI_HTTPS: &str = "https://schema.org/Nonprofit501e";
/// <https://schema.org/Nonprofit501e>
pub const NONPROFIT_501_E_LABEL: &str = "Nonprofit501e";
pub struct Nonprofit501EIri;
impl PartialEq<&str> for Nonprofit501EIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_501_E_IRI_HTTP || *other == NONPROFIT_501_E_IRI_HTTPS
	}
}
impl PartialEq<Nonprofit501EIri> for &str {
	fn eq(&self, other: &Nonprofit501EIri) -> bool {
		*self == NONPROFIT_501_E_IRI_HTTP || *self == NONPROFIT_501_E_IRI_HTTPS
	}
}
pub struct Nonprofit501EIriOrLabel;
impl PartialEq<&str> for Nonprofit501EIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Nonprofit501EIri || *other == NONPROFIT_501_E_LABEL
	}
}
impl PartialEq<Nonprofit501EIriOrLabel> for &str {
	fn eq(&self, other: &Nonprofit501EIriOrLabel) -> bool {
		*self == Nonprofit501EIri || *self == NONPROFIT_501_E_LABEL
	}
}
