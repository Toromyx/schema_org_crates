/// <https://schema.org/passengerPriorityStatus>
pub const PASSENGER_PRIORITY_STATUS_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/passengerPriorityStatus";
/// <https://schema.org/passengerPriorityStatus>
pub const PASSENGER_PRIORITY_STATUS_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/passengerPriorityStatus";
/// <https://schema.org/passengerPriorityStatus>
pub const PASSENGER_PRIORITY_STATUS_PROPERTY_LABEL: &str = "passengerPriorityStatus";
pub struct PassengerPriorityStatusPropertyIri;
impl PartialEq<&str> for PassengerPriorityStatusPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PASSENGER_PRIORITY_STATUS_PROPERTY_IRI_HTTP
			|| *other == PASSENGER_PRIORITY_STATUS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PassengerPriorityStatusPropertyIri> for &str {
	fn eq(&self, other: &PassengerPriorityStatusPropertyIri) -> bool {
		*self == PASSENGER_PRIORITY_STATUS_PROPERTY_IRI_HTTP
			|| *self == PASSENGER_PRIORITY_STATUS_PROPERTY_IRI_HTTPS
	}
}
pub struct PassengerPriorityStatusPropertyIriOrLabel;
impl PartialEq<&str> for PassengerPriorityStatusPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PassengerPriorityStatusPropertyIri
			|| *other == PASSENGER_PRIORITY_STATUS_PROPERTY_LABEL
	}
}
impl PartialEq<PassengerPriorityStatusPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PassengerPriorityStatusPropertyIriOrLabel) -> bool {
		*self == PassengerPriorityStatusPropertyIri
			|| *self == PASSENGER_PRIORITY_STATUS_PROPERTY_LABEL
	}
}
