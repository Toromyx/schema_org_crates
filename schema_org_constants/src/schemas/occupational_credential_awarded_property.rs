/// <https://schema.org/occupationalCredentialAwarded>
pub const OCCUPATIONAL_CREDENTIAL_AWARDED_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/occupationalCredentialAwarded";
/// <https://schema.org/occupationalCredentialAwarded>
pub const OCCUPATIONAL_CREDENTIAL_AWARDED_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/occupationalCredentialAwarded";
/// <https://schema.org/occupationalCredentialAwarded>
pub const OCCUPATIONAL_CREDENTIAL_AWARDED_PROPERTY_LABEL: &str = "occupationalCredentialAwarded";
pub struct OccupationalCredentialAwardedPropertyIri;
impl PartialEq<&str> for OccupationalCredentialAwardedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OCCUPATIONAL_CREDENTIAL_AWARDED_PROPERTY_IRI_HTTP
			|| *other == OCCUPATIONAL_CREDENTIAL_AWARDED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<OccupationalCredentialAwardedPropertyIri> for &str {
	fn eq(&self, other: &OccupationalCredentialAwardedPropertyIri) -> bool {
		*self == OCCUPATIONAL_CREDENTIAL_AWARDED_PROPERTY_IRI_HTTP
			|| *self == OCCUPATIONAL_CREDENTIAL_AWARDED_PROPERTY_IRI_HTTPS
	}
}
pub struct OccupationalCredentialAwardedPropertyIriOrLabel;
impl PartialEq<&str> for OccupationalCredentialAwardedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OccupationalCredentialAwardedPropertyIri
			|| *other == OCCUPATIONAL_CREDENTIAL_AWARDED_PROPERTY_LABEL
	}
}
impl PartialEq<OccupationalCredentialAwardedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &OccupationalCredentialAwardedPropertyIriOrLabel) -> bool {
		*self == OccupationalCredentialAwardedPropertyIri
			|| *self == OCCUPATIONAL_CREDENTIAL_AWARDED_PROPERTY_LABEL
	}
}
