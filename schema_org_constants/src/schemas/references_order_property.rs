/// <https://schema.org/referencesOrder>
pub const REFERENCES_ORDER_PROPERTY_IRI_HTTP: &str = "http://schema.org/referencesOrder";
/// <https://schema.org/referencesOrder>
pub const REFERENCES_ORDER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/referencesOrder";
/// <https://schema.org/referencesOrder>
pub const REFERENCES_ORDER_PROPERTY_LABEL: &str = "referencesOrder";
pub struct ReferencesOrderPropertyIri;
impl PartialEq<&str> for ReferencesOrderPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REFERENCES_ORDER_PROPERTY_IRI_HTTP
			|| *other == REFERENCES_ORDER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ReferencesOrderPropertyIri> for &str {
	fn eq(&self, other: &ReferencesOrderPropertyIri) -> bool {
		*self == REFERENCES_ORDER_PROPERTY_IRI_HTTP || *self == REFERENCES_ORDER_PROPERTY_IRI_HTTPS
	}
}
pub struct ReferencesOrderPropertyIriOrLabel;
impl PartialEq<&str> for ReferencesOrderPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReferencesOrderPropertyIri || *other == REFERENCES_ORDER_PROPERTY_LABEL
	}
}
impl PartialEq<ReferencesOrderPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ReferencesOrderPropertyIriOrLabel) -> bool {
		*self == ReferencesOrderPropertyIri || *self == REFERENCES_ORDER_PROPERTY_LABEL
	}
}
