/// <https://schema.org/Nonprofit501n>
pub const NONPROFIT_501_N_IRI_HTTP: &str = "http://schema.org/Nonprofit501n";
/// <https://schema.org/Nonprofit501n>
pub const NONPROFIT_501_N_IRI_HTTPS: &str = "https://schema.org/Nonprofit501n";
/// <https://schema.org/Nonprofit501n>
pub const NONPROFIT_501_N_LABEL: &str = "Nonprofit501n";
pub struct Nonprofit501NIri;
impl PartialEq<&str> for Nonprofit501NIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_501_N_IRI_HTTP || *other == NONPROFIT_501_N_IRI_HTTPS
	}
}
impl PartialEq<Nonprofit501NIri> for &str {
	fn eq(&self, other: &Nonprofit501NIri) -> bool {
		*self == NONPROFIT_501_N_IRI_HTTP || *self == NONPROFIT_501_N_IRI_HTTPS
	}
}
pub struct Nonprofit501NIriOrLabel;
impl PartialEq<&str> for Nonprofit501NIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Nonprofit501NIri || *other == NONPROFIT_501_N_LABEL
	}
}
impl PartialEq<Nonprofit501NIriOrLabel> for &str {
	fn eq(&self, other: &Nonprofit501NIriOrLabel) -> bool {
		*self == Nonprofit501NIri || *self == NONPROFIT_501_N_LABEL
	}
}
