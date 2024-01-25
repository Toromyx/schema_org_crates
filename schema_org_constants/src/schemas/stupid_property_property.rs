/// <https://schema.org/stupidProperty>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>."]
pub const STUPID_PROPERTY_PROPERTY_IRI_HTTP: &str = "http://schema.org/stupidProperty";
/// <https://schema.org/stupidProperty>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>."]
pub const STUPID_PROPERTY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/stupidProperty";
/// <https://schema.org/stupidProperty>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>."]
pub const STUPID_PROPERTY_PROPERTY_LABEL: &str = "stupidProperty";
pub struct StupidPropertyPropertyIri;
impl PartialEq<&str> for StupidPropertyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == STUPID_PROPERTY_PROPERTY_IRI_HTTP || *other == STUPID_PROPERTY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<StupidPropertyPropertyIri> for &str {
	fn eq(&self, other: &StupidPropertyPropertyIri) -> bool {
		*self == STUPID_PROPERTY_PROPERTY_IRI_HTTP || *self == STUPID_PROPERTY_PROPERTY_IRI_HTTPS
	}
}
pub struct StupidPropertyPropertyIriOrLabel;
impl PartialEq<&str> for StupidPropertyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == StupidPropertyPropertyIri || *other == STUPID_PROPERTY_PROPERTY_LABEL
	}
}
impl PartialEq<StupidPropertyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &StupidPropertyPropertyIriOrLabel) -> bool {
		*self == StupidPropertyPropertyIri || *self == STUPID_PROPERTY_PROPERTY_LABEL
	}
}
