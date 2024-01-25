/// <https://schema.org/RentAction>
pub const RENT_ACTION_IRI_HTTP: &str = "http://schema.org/RentAction";
/// <https://schema.org/RentAction>
pub const RENT_ACTION_IRI_HTTPS: &str = "https://schema.org/RentAction";
/// <https://schema.org/RentAction>
pub const RENT_ACTION_LABEL: &str = "RentAction";
pub struct RentActionIri;
impl PartialEq<&str> for RentActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RENT_ACTION_IRI_HTTP || *other == RENT_ACTION_IRI_HTTPS
	}
}
impl PartialEq<RentActionIri> for &str {
	fn eq(&self, other: &RentActionIri) -> bool {
		*self == RENT_ACTION_IRI_HTTP || *self == RENT_ACTION_IRI_HTTPS
	}
}
pub struct RentActionIriOrLabel;
impl PartialEq<&str> for RentActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RentActionIri || *other == RENT_ACTION_LABEL
	}
}
impl PartialEq<RentActionIriOrLabel> for &str {
	fn eq(&self, other: &RentActionIriOrLabel) -> bool {
		*self == RentActionIri || *self == RENT_ACTION_LABEL
	}
}
