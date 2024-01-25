/// <https://schema.org/Nonprofit501f>
pub const NONPROFIT_501_F_IRI_HTTP: &str = "http://schema.org/Nonprofit501f";
/// <https://schema.org/Nonprofit501f>
pub const NONPROFIT_501_F_IRI_HTTPS: &str = "https://schema.org/Nonprofit501f";
/// <https://schema.org/Nonprofit501f>
pub const NONPROFIT_501_F_LABEL: &str = "Nonprofit501f";
pub struct Nonprofit501FIri;
impl PartialEq<&str> for Nonprofit501FIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_501_F_IRI_HTTP || *other == NONPROFIT_501_F_IRI_HTTPS
	}
}
impl PartialEq<Nonprofit501FIri> for &str {
	fn eq(&self, other: &Nonprofit501FIri) -> bool {
		*self == NONPROFIT_501_F_IRI_HTTP || *self == NONPROFIT_501_F_IRI_HTTPS
	}
}
pub struct Nonprofit501FIriOrLabel;
impl PartialEq<&str> for Nonprofit501FIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Nonprofit501FIri || *other == NONPROFIT_501_F_LABEL
	}
}
impl PartialEq<Nonprofit501FIriOrLabel> for &str {
	fn eq(&self, other: &Nonprofit501FIriOrLabel) -> bool {
		*self == Nonprofit501FIri || *self == NONPROFIT_501_F_LABEL
	}
}
