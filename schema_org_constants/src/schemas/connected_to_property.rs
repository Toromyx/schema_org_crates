/// <https://schema.org/connectedTo>
pub const CONNECTED_TO_PROPERTY_IRI_HTTP: &str = "http://schema.org/connectedTo";
/// <https://schema.org/connectedTo>
pub const CONNECTED_TO_PROPERTY_IRI_HTTPS: &str = "https://schema.org/connectedTo";
/// <https://schema.org/connectedTo>
pub const CONNECTED_TO_PROPERTY_LABEL: &str = "connectedTo";
pub struct ConnectedToPropertyIri;
impl PartialEq<&str> for ConnectedToPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CONNECTED_TO_PROPERTY_IRI_HTTP || *other == CONNECTED_TO_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ConnectedToPropertyIri> for &str {
	fn eq(&self, other: &ConnectedToPropertyIri) -> bool {
		*self == CONNECTED_TO_PROPERTY_IRI_HTTP || *self == CONNECTED_TO_PROPERTY_IRI_HTTPS
	}
}
pub struct ConnectedToPropertyIriOrLabel;
impl PartialEq<&str> for ConnectedToPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ConnectedToPropertyIri || *other == CONNECTED_TO_PROPERTY_LABEL
	}
}
impl PartialEq<ConnectedToPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ConnectedToPropertyIriOrLabel) -> bool {
		*self == ConnectedToPropertyIri || *self == CONNECTED_TO_PROPERTY_LABEL
	}
}
