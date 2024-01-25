/// <https://schema.org/WesternConventional>
pub const WESTERN_CONVENTIONAL_IRI_HTTP: &str = "http://schema.org/WesternConventional";
/// <https://schema.org/WesternConventional>
pub const WESTERN_CONVENTIONAL_IRI_HTTPS: &str = "https://schema.org/WesternConventional";
/// <https://schema.org/WesternConventional>
pub const WESTERN_CONVENTIONAL_LABEL: &str = "WesternConventional";
pub struct WesternConventionalIri;
impl PartialEq<&str> for WesternConventionalIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WESTERN_CONVENTIONAL_IRI_HTTP || *other == WESTERN_CONVENTIONAL_IRI_HTTPS
	}
}
impl PartialEq<WesternConventionalIri> for &str {
	fn eq(&self, other: &WesternConventionalIri) -> bool {
		*self == WESTERN_CONVENTIONAL_IRI_HTTP || *self == WESTERN_CONVENTIONAL_IRI_HTTPS
	}
}
pub struct WesternConventionalIriOrLabel;
impl PartialEq<&str> for WesternConventionalIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WesternConventionalIri || *other == WESTERN_CONVENTIONAL_LABEL
	}
}
impl PartialEq<WesternConventionalIriOrLabel> for &str {
	fn eq(&self, other: &WesternConventionalIriOrLabel) -> bool {
		*self == WesternConventionalIri || *self == WESTERN_CONVENTIONAL_LABEL
	}
}
