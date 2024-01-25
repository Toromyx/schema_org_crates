/// <https://schema.org/MonetaryGrant>
pub const MONETARY_GRANT_IRI_HTTP: &str = "http://schema.org/MonetaryGrant";
/// <https://schema.org/MonetaryGrant>
pub const MONETARY_GRANT_IRI_HTTPS: &str = "https://schema.org/MonetaryGrant";
/// <https://schema.org/MonetaryGrant>
pub const MONETARY_GRANT_LABEL: &str = "MonetaryGrant";
pub struct MonetaryGrantIri;
impl PartialEq<&str> for MonetaryGrantIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MONETARY_GRANT_IRI_HTTP || *other == MONETARY_GRANT_IRI_HTTPS
	}
}
impl PartialEq<MonetaryGrantIri> for &str {
	fn eq(&self, other: &MonetaryGrantIri) -> bool {
		*self == MONETARY_GRANT_IRI_HTTP || *self == MONETARY_GRANT_IRI_HTTPS
	}
}
pub struct MonetaryGrantIriOrLabel;
impl PartialEq<&str> for MonetaryGrantIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MonetaryGrantIri || *other == MONETARY_GRANT_LABEL
	}
}
impl PartialEq<MonetaryGrantIriOrLabel> for &str {
	fn eq(&self, other: &MonetaryGrantIriOrLabel) -> bool {
		*self == MonetaryGrantIri || *self == MONETARY_GRANT_LABEL
	}
}
