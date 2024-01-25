/// <https://schema.org/letterer>
pub const LETTERER_PROPERTY_IRI_HTTP: &str = "http://schema.org/letterer";
/// <https://schema.org/letterer>
pub const LETTERER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/letterer";
/// <https://schema.org/letterer>
pub const LETTERER_PROPERTY_LABEL: &str = "letterer";
pub struct LettererPropertyIri;
impl PartialEq<&str> for LettererPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LETTERER_PROPERTY_IRI_HTTP || *other == LETTERER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LettererPropertyIri> for &str {
	fn eq(&self, other: &LettererPropertyIri) -> bool {
		*self == LETTERER_PROPERTY_IRI_HTTP || *self == LETTERER_PROPERTY_IRI_HTTPS
	}
}
pub struct LettererPropertyIriOrLabel;
impl PartialEq<&str> for LettererPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LettererPropertyIri || *other == LETTERER_PROPERTY_LABEL
	}
}
impl PartialEq<LettererPropertyIriOrLabel> for &str {
	fn eq(&self, other: &LettererPropertyIriOrLabel) -> bool {
		*self == LettererPropertyIri || *self == LETTERER_PROPERTY_LABEL
	}
}
