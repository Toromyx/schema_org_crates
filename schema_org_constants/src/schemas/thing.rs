/// <https://schema.org/Thing>
pub const THING_IRI_HTTP: &str = "http://schema.org/Thing";
/// <https://schema.org/Thing>
pub const THING_IRI_HTTPS: &str = "https://schema.org/Thing";
/// <https://schema.org/Thing>
pub const THING_LABEL: &str = "Thing";
pub struct ThingIri;
impl PartialEq<&str> for ThingIri {
	fn eq(&self, other: &&str) -> bool {
		*other == THING_IRI_HTTP || *other == THING_IRI_HTTPS
	}
}
impl PartialEq<ThingIri> for &str {
	fn eq(&self, other: &ThingIri) -> bool {
		*self == THING_IRI_HTTP || *self == THING_IRI_HTTPS
	}
}
pub struct ThingIriOrLabel;
impl PartialEq<&str> for ThingIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ThingIri || *other == THING_LABEL
	}
}
impl PartialEq<ThingIriOrLabel> for &str {
	fn eq(&self, other: &ThingIriOrLabel) -> bool {
		*self == ThingIri || *self == THING_LABEL
	}
}
