/// <https://schema.org/eligibleTransactionVolume>
pub const ELIGIBLE_TRANSACTION_VOLUME_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/eligibleTransactionVolume";
/// <https://schema.org/eligibleTransactionVolume>
pub const ELIGIBLE_TRANSACTION_VOLUME_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/eligibleTransactionVolume";
/// <https://schema.org/eligibleTransactionVolume>
pub const ELIGIBLE_TRANSACTION_VOLUME_PROPERTY_LABEL: &str = "eligibleTransactionVolume";
pub struct EligibleTransactionVolumePropertyIri;
impl PartialEq<&str> for EligibleTransactionVolumePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ELIGIBLE_TRANSACTION_VOLUME_PROPERTY_IRI_HTTP
			|| *other == ELIGIBLE_TRANSACTION_VOLUME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EligibleTransactionVolumePropertyIri> for &str {
	fn eq(&self, other: &EligibleTransactionVolumePropertyIri) -> bool {
		*self == ELIGIBLE_TRANSACTION_VOLUME_PROPERTY_IRI_HTTP
			|| *self == ELIGIBLE_TRANSACTION_VOLUME_PROPERTY_IRI_HTTPS
	}
}
pub struct EligibleTransactionVolumePropertyIriOrLabel;
impl PartialEq<&str> for EligibleTransactionVolumePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EligibleTransactionVolumePropertyIri
			|| *other == ELIGIBLE_TRANSACTION_VOLUME_PROPERTY_LABEL
	}
}
impl PartialEq<EligibleTransactionVolumePropertyIriOrLabel> for &str {
	fn eq(&self, other: &EligibleTransactionVolumePropertyIriOrLabel) -> bool {
		*self == EligibleTransactionVolumePropertyIri
			|| *self == ELIGIBLE_TRANSACTION_VOLUME_PROPERTY_LABEL
	}
}
