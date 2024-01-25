/// <https://schema.org/passengerSequenceNumber>
pub const PASSENGER_SEQUENCE_NUMBER_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/passengerSequenceNumber";
/// <https://schema.org/passengerSequenceNumber>
pub const PASSENGER_SEQUENCE_NUMBER_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/passengerSequenceNumber";
/// <https://schema.org/passengerSequenceNumber>
pub const PASSENGER_SEQUENCE_NUMBER_PROPERTY_LABEL: &str = "passengerSequenceNumber";
pub struct PassengerSequenceNumberPropertyIri;
impl PartialEq<&str> for PassengerSequenceNumberPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PASSENGER_SEQUENCE_NUMBER_PROPERTY_IRI_HTTP
			|| *other == PASSENGER_SEQUENCE_NUMBER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PassengerSequenceNumberPropertyIri> for &str {
	fn eq(&self, other: &PassengerSequenceNumberPropertyIri) -> bool {
		*self == PASSENGER_SEQUENCE_NUMBER_PROPERTY_IRI_HTTP
			|| *self == PASSENGER_SEQUENCE_NUMBER_PROPERTY_IRI_HTTPS
	}
}
pub struct PassengerSequenceNumberPropertyIriOrLabel;
impl PartialEq<&str> for PassengerSequenceNumberPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PassengerSequenceNumberPropertyIri
			|| *other == PASSENGER_SEQUENCE_NUMBER_PROPERTY_LABEL
	}
}
impl PartialEq<PassengerSequenceNumberPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PassengerSequenceNumberPropertyIriOrLabel) -> bool {
		*self == PassengerSequenceNumberPropertyIri
			|| *self == PASSENGER_SEQUENCE_NUMBER_PROPERTY_LABEL
	}
}
