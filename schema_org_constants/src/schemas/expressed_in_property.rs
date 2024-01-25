/// <https://schema.org/expressedIn>
pub const EXPRESSED_IN_PROPERTY_IRI_HTTP: &str = "http://schema.org/expressedIn";
/// <https://schema.org/expressedIn>
pub const EXPRESSED_IN_PROPERTY_IRI_HTTPS: &str = "https://schema.org/expressedIn";
/// <https://schema.org/expressedIn>
pub const EXPRESSED_IN_PROPERTY_LABEL: &str = "expressedIn";
pub struct ExpressedInPropertyIri;
impl PartialEq<&str> for ExpressedInPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EXPRESSED_IN_PROPERTY_IRI_HTTP || *other == EXPRESSED_IN_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ExpressedInPropertyIri> for &str {
	fn eq(&self, other: &ExpressedInPropertyIri) -> bool {
		*self == EXPRESSED_IN_PROPERTY_IRI_HTTP || *self == EXPRESSED_IN_PROPERTY_IRI_HTTPS
	}
}
pub struct ExpressedInPropertyIriOrLabel;
impl PartialEq<&str> for ExpressedInPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ExpressedInPropertyIri || *other == EXPRESSED_IN_PROPERTY_LABEL
	}
}
impl PartialEq<ExpressedInPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ExpressedInPropertyIriOrLabel) -> bool {
		*self == ExpressedInPropertyIri || *self == EXPRESSED_IN_PROPERTY_LABEL
	}
}
