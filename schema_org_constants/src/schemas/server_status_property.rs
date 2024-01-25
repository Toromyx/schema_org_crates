/// <https://schema.org/serverStatus>
pub const SERVER_STATUS_PROPERTY_IRI_HTTP: &str = "http://schema.org/serverStatus";
/// <https://schema.org/serverStatus>
pub const SERVER_STATUS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/serverStatus";
/// <https://schema.org/serverStatus>
pub const SERVER_STATUS_PROPERTY_LABEL: &str = "serverStatus";
pub struct ServerStatusPropertyIri;
impl PartialEq<&str> for ServerStatusPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SERVER_STATUS_PROPERTY_IRI_HTTP || *other == SERVER_STATUS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ServerStatusPropertyIri> for &str {
	fn eq(&self, other: &ServerStatusPropertyIri) -> bool {
		*self == SERVER_STATUS_PROPERTY_IRI_HTTP || *self == SERVER_STATUS_PROPERTY_IRI_HTTPS
	}
}
pub struct ServerStatusPropertyIriOrLabel;
impl PartialEq<&str> for ServerStatusPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ServerStatusPropertyIri || *other == SERVER_STATUS_PROPERTY_LABEL
	}
}
impl PartialEq<ServerStatusPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ServerStatusPropertyIriOrLabel) -> bool {
		*self == ServerStatusPropertyIri || *self == SERVER_STATUS_PROPERTY_LABEL
	}
}
