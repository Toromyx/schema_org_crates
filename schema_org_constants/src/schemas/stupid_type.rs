/// <https://schema.org/StupidType>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>."]
pub const STUPID_TYPE_IRI_HTTP: &str = "http://schema.org/StupidType";
/// <https://schema.org/StupidType>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>."]
pub const STUPID_TYPE_IRI_HTTPS: &str = "https://schema.org/StupidType";
/// <https://schema.org/StupidType>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>."]
pub const STUPID_TYPE_LABEL: &str = "StupidType";
pub struct StupidTypeIri;
impl PartialEq<&str> for StupidTypeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == STUPID_TYPE_IRI_HTTP || *other == STUPID_TYPE_IRI_HTTPS
	}
}
impl PartialEq<StupidTypeIri> for &str {
	fn eq(&self, other: &StupidTypeIri) -> bool {
		*self == STUPID_TYPE_IRI_HTTP || *self == STUPID_TYPE_IRI_HTTPS
	}
}
pub struct StupidTypeIriOrLabel;
impl PartialEq<&str> for StupidTypeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == StupidTypeIri || *other == STUPID_TYPE_LABEL
	}
}
impl PartialEq<StupidTypeIriOrLabel> for &str {
	fn eq(&self, other: &StupidTypeIriOrLabel) -> bool {
		*self == StupidTypeIri || *self == STUPID_TYPE_LABEL
	}
}
