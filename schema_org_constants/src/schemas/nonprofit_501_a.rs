/// <https://schema.org/Nonprofit501a>
pub const NONPROFIT_501_A_IRI_HTTP: &str = "http://schema.org/Nonprofit501a";
/// <https://schema.org/Nonprofit501a>
pub const NONPROFIT_501_A_IRI_HTTPS: &str = "https://schema.org/Nonprofit501a";
/// <https://schema.org/Nonprofit501a>
pub const NONPROFIT_501_A_LABEL: &str = "Nonprofit501a";
pub struct Nonprofit501AIri;
impl PartialEq<&str> for Nonprofit501AIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_501_A_IRI_HTTP || *other == NONPROFIT_501_A_IRI_HTTPS
	}
}
impl PartialEq<Nonprofit501AIri> for &str {
	fn eq(&self, other: &Nonprofit501AIri) -> bool {
		*self == NONPROFIT_501_A_IRI_HTTP || *self == NONPROFIT_501_A_IRI_HTTPS
	}
}
pub struct Nonprofit501AIriOrLabel;
impl PartialEq<&str> for Nonprofit501AIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Nonprofit501AIri || *other == NONPROFIT_501_A_LABEL
	}
}
impl PartialEq<Nonprofit501AIriOrLabel> for &str {
	fn eq(&self, other: &Nonprofit501AIriOrLabel) -> bool {
		*self == Nonprofit501AIri || *self == NONPROFIT_501_A_LABEL
	}
}
