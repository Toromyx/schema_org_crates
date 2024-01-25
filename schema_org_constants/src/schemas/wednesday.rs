/// <https://schema.org/Wednesday>
pub const WEDNESDAY_IRI_HTTP: &str = "http://schema.org/Wednesday";
/// <https://schema.org/Wednesday>
pub const WEDNESDAY_IRI_HTTPS: &str = "https://schema.org/Wednesday";
/// <https://schema.org/Wednesday>
pub const WEDNESDAY_LABEL: &str = "Wednesday";
pub struct WednesdayIri;
impl PartialEq<&str> for WednesdayIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEDNESDAY_IRI_HTTP || *other == WEDNESDAY_IRI_HTTPS
	}
}
impl PartialEq<WednesdayIri> for &str {
	fn eq(&self, other: &WednesdayIri) -> bool {
		*self == WEDNESDAY_IRI_HTTP || *self == WEDNESDAY_IRI_HTTPS
	}
}
pub struct WednesdayIriOrLabel;
impl PartialEq<&str> for WednesdayIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WednesdayIri || *other == WEDNESDAY_LABEL
	}
}
impl PartialEq<WednesdayIriOrLabel> for &str {
	fn eq(&self, other: &WednesdayIriOrLabel) -> bool {
		*self == WednesdayIri || *self == WEDNESDAY_LABEL
	}
}
