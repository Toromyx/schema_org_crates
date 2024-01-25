/// <https://schema.org/netWorth>
pub const NET_WORTH_PROPERTY_IRI_HTTP: &str = "http://schema.org/netWorth";
/// <https://schema.org/netWorth>
pub const NET_WORTH_PROPERTY_IRI_HTTPS: &str = "https://schema.org/netWorth";
/// <https://schema.org/netWorth>
pub const NET_WORTH_PROPERTY_LABEL: &str = "netWorth";
pub struct NetWorthPropertyIri;
impl PartialEq<&str> for NetWorthPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NET_WORTH_PROPERTY_IRI_HTTP || *other == NET_WORTH_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NetWorthPropertyIri> for &str {
	fn eq(&self, other: &NetWorthPropertyIri) -> bool {
		*self == NET_WORTH_PROPERTY_IRI_HTTP || *self == NET_WORTH_PROPERTY_IRI_HTTPS
	}
}
pub struct NetWorthPropertyIriOrLabel;
impl PartialEq<&str> for NetWorthPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NetWorthPropertyIri || *other == NET_WORTH_PROPERTY_LABEL
	}
}
impl PartialEq<NetWorthPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NetWorthPropertyIriOrLabel) -> bool {
		*self == NetWorthPropertyIri || *self == NET_WORTH_PROPERTY_LABEL
	}
}
