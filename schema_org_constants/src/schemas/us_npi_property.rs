/// <https://schema.org/usNPI>
pub const US_NPI_PROPERTY_IRI_HTTP: &str = "http://schema.org/usNPI";
/// <https://schema.org/usNPI>
pub const US_NPI_PROPERTY_IRI_HTTPS: &str = "https://schema.org/usNPI";
/// <https://schema.org/usNPI>
pub const US_NPI_PROPERTY_LABEL: &str = "usNPI";
pub struct UsNpiPropertyIri;
impl PartialEq<&str> for UsNpiPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == US_NPI_PROPERTY_IRI_HTTP || *other == US_NPI_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<UsNpiPropertyIri> for &str {
	fn eq(&self, other: &UsNpiPropertyIri) -> bool {
		*self == US_NPI_PROPERTY_IRI_HTTP || *self == US_NPI_PROPERTY_IRI_HTTPS
	}
}
pub struct UsNpiPropertyIriOrLabel;
impl PartialEq<&str> for UsNpiPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UsNpiPropertyIri || *other == US_NPI_PROPERTY_LABEL
	}
}
impl PartialEq<UsNpiPropertyIriOrLabel> for &str {
	fn eq(&self, other: &UsNpiPropertyIriOrLabel) -> bool {
		*self == UsNpiPropertyIri || *self == US_NPI_PROPERTY_LABEL
	}
}
