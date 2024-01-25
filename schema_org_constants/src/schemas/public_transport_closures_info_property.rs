/// <https://schema.org/publicTransportClosuresInfo>
pub const PUBLIC_TRANSPORT_CLOSURES_INFO_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/publicTransportClosuresInfo";
/// <https://schema.org/publicTransportClosuresInfo>
pub const PUBLIC_TRANSPORT_CLOSURES_INFO_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/publicTransportClosuresInfo";
/// <https://schema.org/publicTransportClosuresInfo>
pub const PUBLIC_TRANSPORT_CLOSURES_INFO_PROPERTY_LABEL: &str = "publicTransportClosuresInfo";
pub struct PublicTransportClosuresInfoPropertyIri;
impl PartialEq<&str> for PublicTransportClosuresInfoPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PUBLIC_TRANSPORT_CLOSURES_INFO_PROPERTY_IRI_HTTP
			|| *other == PUBLIC_TRANSPORT_CLOSURES_INFO_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PublicTransportClosuresInfoPropertyIri> for &str {
	fn eq(&self, other: &PublicTransportClosuresInfoPropertyIri) -> bool {
		*self == PUBLIC_TRANSPORT_CLOSURES_INFO_PROPERTY_IRI_HTTP
			|| *self == PUBLIC_TRANSPORT_CLOSURES_INFO_PROPERTY_IRI_HTTPS
	}
}
pub struct PublicTransportClosuresInfoPropertyIriOrLabel;
impl PartialEq<&str> for PublicTransportClosuresInfoPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PublicTransportClosuresInfoPropertyIri
			|| *other == PUBLIC_TRANSPORT_CLOSURES_INFO_PROPERTY_LABEL
	}
}
impl PartialEq<PublicTransportClosuresInfoPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PublicTransportClosuresInfoPropertyIriOrLabel) -> bool {
		*self == PublicTransportClosuresInfoPropertyIri
			|| *self == PUBLIC_TRANSPORT_CLOSURES_INFO_PROPERTY_LABEL
	}
}
