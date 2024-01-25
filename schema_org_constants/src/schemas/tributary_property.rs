/// <https://schema.org/tributary>
pub const TRIBUTARY_PROPERTY_IRI_HTTP: &str = "http://schema.org/tributary";
/// <https://schema.org/tributary>
pub const TRIBUTARY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/tributary";
/// <https://schema.org/tributary>
pub const TRIBUTARY_PROPERTY_LABEL: &str = "tributary";
pub struct TributaryPropertyIri;
impl PartialEq<&str> for TributaryPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TRIBUTARY_PROPERTY_IRI_HTTP || *other == TRIBUTARY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TributaryPropertyIri> for &str {
	fn eq(&self, other: &TributaryPropertyIri) -> bool {
		*self == TRIBUTARY_PROPERTY_IRI_HTTP || *self == TRIBUTARY_PROPERTY_IRI_HTTPS
	}
}
pub struct TributaryPropertyIriOrLabel;
impl PartialEq<&str> for TributaryPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TributaryPropertyIri || *other == TRIBUTARY_PROPERTY_LABEL
	}
}
impl PartialEq<TributaryPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TributaryPropertyIriOrLabel) -> bool {
		*self == TributaryPropertyIri || *self == TRIBUTARY_PROPERTY_LABEL
	}
}
