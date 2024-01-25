/// <https://schema.org/landlord>
pub const LANDLORD_PROPERTY_IRI_HTTP: &str = "http://schema.org/landlord";
/// <https://schema.org/landlord>
pub const LANDLORD_PROPERTY_IRI_HTTPS: &str = "https://schema.org/landlord";
/// <https://schema.org/landlord>
pub const LANDLORD_PROPERTY_LABEL: &str = "landlord";
pub struct LandlordPropertyIri;
impl PartialEq<&str> for LandlordPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LANDLORD_PROPERTY_IRI_HTTP || *other == LANDLORD_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LandlordPropertyIri> for &str {
	fn eq(&self, other: &LandlordPropertyIri) -> bool {
		*self == LANDLORD_PROPERTY_IRI_HTTP || *self == LANDLORD_PROPERTY_IRI_HTTPS
	}
}
pub struct LandlordPropertyIriOrLabel;
impl PartialEq<&str> for LandlordPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LandlordPropertyIri || *other == LANDLORD_PROPERTY_LABEL
	}
}
impl PartialEq<LandlordPropertyIriOrLabel> for &str {
	fn eq(&self, other: &LandlordPropertyIriOrLabel) -> bool {
		*self == LandlordPropertyIri || *self == LANDLORD_PROPERTY_LABEL
	}
}
