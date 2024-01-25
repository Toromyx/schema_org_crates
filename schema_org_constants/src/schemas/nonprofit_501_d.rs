/// <https://schema.org/Nonprofit501d>
pub const NONPROFIT_501_D_IRI_HTTP: &str = "http://schema.org/Nonprofit501d";
/// <https://schema.org/Nonprofit501d>
pub const NONPROFIT_501_D_IRI_HTTPS: &str = "https://schema.org/Nonprofit501d";
/// <https://schema.org/Nonprofit501d>
pub const NONPROFIT_501_D_LABEL: &str = "Nonprofit501d";
pub struct Nonprofit501DIri;
impl PartialEq<&str> for Nonprofit501DIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_501_D_IRI_HTTP || *other == NONPROFIT_501_D_IRI_HTTPS
	}
}
impl PartialEq<Nonprofit501DIri> for &str {
	fn eq(&self, other: &Nonprofit501DIri) -> bool {
		*self == NONPROFIT_501_D_IRI_HTTP || *self == NONPROFIT_501_D_IRI_HTTPS
	}
}
pub struct Nonprofit501DIriOrLabel;
impl PartialEq<&str> for Nonprofit501DIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Nonprofit501DIri || *other == NONPROFIT_501_D_LABEL
	}
}
impl PartialEq<Nonprofit501DIriOrLabel> for &str {
	fn eq(&self, other: &Nonprofit501DIriOrLabel) -> bool {
		*self == Nonprofit501DIri || *self == NONPROFIT_501_D_LABEL
	}
}
