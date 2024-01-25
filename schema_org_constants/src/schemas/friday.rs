/// <https://schema.org/Friday>
pub const FRIDAY_IRI_HTTP: &str = "http://schema.org/Friday";
/// <https://schema.org/Friday>
pub const FRIDAY_IRI_HTTPS: &str = "https://schema.org/Friday";
/// <https://schema.org/Friday>
pub const FRIDAY_LABEL: &str = "Friday";
pub struct FridayIri;
impl PartialEq<&str> for FridayIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FRIDAY_IRI_HTTP || *other == FRIDAY_IRI_HTTPS
	}
}
impl PartialEq<FridayIri> for &str {
	fn eq(&self, other: &FridayIri) -> bool {
		*self == FRIDAY_IRI_HTTP || *self == FRIDAY_IRI_HTTPS
	}
}
pub struct FridayIriOrLabel;
impl PartialEq<&str> for FridayIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FridayIri || *other == FRIDAY_LABEL
	}
}
impl PartialEq<FridayIriOrLabel> for &str {
	fn eq(&self, other: &FridayIriOrLabel) -> bool {
		*self == FridayIri || *self == FRIDAY_LABEL
	}
}
