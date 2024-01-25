/// <https://schema.org/BroadcastFrequencySpecification>
pub const BROADCAST_FREQUENCY_SPECIFICATION_IRI_HTTP: &str =
	"http://schema.org/BroadcastFrequencySpecification";
/// <https://schema.org/BroadcastFrequencySpecification>
pub const BROADCAST_FREQUENCY_SPECIFICATION_IRI_HTTPS: &str =
	"https://schema.org/BroadcastFrequencySpecification";
/// <https://schema.org/BroadcastFrequencySpecification>
pub const BROADCAST_FREQUENCY_SPECIFICATION_LABEL: &str = "BroadcastFrequencySpecification";
pub struct BroadcastFrequencySpecificationIri;
impl PartialEq<&str> for BroadcastFrequencySpecificationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BROADCAST_FREQUENCY_SPECIFICATION_IRI_HTTP
			|| *other == BROADCAST_FREQUENCY_SPECIFICATION_IRI_HTTPS
	}
}
impl PartialEq<BroadcastFrequencySpecificationIri> for &str {
	fn eq(&self, other: &BroadcastFrequencySpecificationIri) -> bool {
		*self == BROADCAST_FREQUENCY_SPECIFICATION_IRI_HTTP
			|| *self == BROADCAST_FREQUENCY_SPECIFICATION_IRI_HTTPS
	}
}
pub struct BroadcastFrequencySpecificationIriOrLabel;
impl PartialEq<&str> for BroadcastFrequencySpecificationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BroadcastFrequencySpecificationIri
			|| *other == BROADCAST_FREQUENCY_SPECIFICATION_LABEL
	}
}
impl PartialEq<BroadcastFrequencySpecificationIriOrLabel> for &str {
	fn eq(&self, other: &BroadcastFrequencySpecificationIriOrLabel) -> bool {
		*self == BroadcastFrequencySpecificationIri
			|| *self == BROADCAST_FREQUENCY_SPECIFICATION_LABEL
	}
}
