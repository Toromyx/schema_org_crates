/// <https://schema.org/Nonprofit501q>
pub const NONPROFIT_501_Q_IRI_HTTP: &str = "http://schema.org/Nonprofit501q";
/// <https://schema.org/Nonprofit501q>
pub const NONPROFIT_501_Q_IRI_HTTPS: &str = "https://schema.org/Nonprofit501q";
/// <https://schema.org/Nonprofit501q>
pub const NONPROFIT_501_Q_LABEL: &str = "Nonprofit501q";
pub struct Nonprofit501QIri;
impl PartialEq<&str> for Nonprofit501QIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_501_Q_IRI_HTTP || *other == NONPROFIT_501_Q_IRI_HTTPS
	}
}
impl PartialEq<Nonprofit501QIri> for &str {
	fn eq(&self, other: &Nonprofit501QIri) -> bool {
		*self == NONPROFIT_501_Q_IRI_HTTP || *self == NONPROFIT_501_Q_IRI_HTTPS
	}
}
pub struct Nonprofit501QIriOrLabel;
impl PartialEq<&str> for Nonprofit501QIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Nonprofit501QIri || *other == NONPROFIT_501_Q_LABEL
	}
}
impl PartialEq<Nonprofit501QIriOrLabel> for &str {
	fn eq(&self, other: &Nonprofit501QIriOrLabel) -> bool {
		*self == Nonprofit501QIri || *self == NONPROFIT_501_Q_LABEL
	}
}
