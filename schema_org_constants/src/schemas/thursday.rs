/// <https://schema.org/Thursday>
pub const THURSDAY_IRI_HTTP: &str = "http://schema.org/Thursday";
/// <https://schema.org/Thursday>
pub const THURSDAY_IRI_HTTPS: &str = "https://schema.org/Thursday";
/// <https://schema.org/Thursday>
pub const THURSDAY_LABEL: &str = "Thursday";
pub struct ThursdayIri;
impl PartialEq<&str> for ThursdayIri {
	fn eq(&self, other: &&str) -> bool {
		*other == THURSDAY_IRI_HTTP || *other == THURSDAY_IRI_HTTPS
	}
}
impl PartialEq<ThursdayIri> for &str {
	fn eq(&self, other: &ThursdayIri) -> bool {
		*self == THURSDAY_IRI_HTTP || *self == THURSDAY_IRI_HTTPS
	}
}
pub struct ThursdayIriOrLabel;
impl PartialEq<&str> for ThursdayIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ThursdayIri || *other == THURSDAY_LABEL
	}
}
impl PartialEq<ThursdayIriOrLabel> for &str {
	fn eq(&self, other: &ThursdayIriOrLabel) -> bool {
		*self == ThursdayIri || *self == THURSDAY_LABEL
	}
}
