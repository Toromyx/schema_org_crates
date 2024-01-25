/// <https://schema.org/ExampleMeasurementMethodEnum>
pub const EXAMPLE_MEASUREMENT_METHOD_ENUM_IRI_HTTP: &str =
	"http://schema.org/ExampleMeasurementMethodEnum";
/// <https://schema.org/ExampleMeasurementMethodEnum>
pub const EXAMPLE_MEASUREMENT_METHOD_ENUM_IRI_HTTPS: &str =
	"https://schema.org/ExampleMeasurementMethodEnum";
/// <https://schema.org/ExampleMeasurementMethodEnum>
pub const EXAMPLE_MEASUREMENT_METHOD_ENUM_LABEL: &str = "ExampleMeasurementMethodEnum";
pub struct ExampleMeasurementMethodEnumIri;
impl PartialEq<&str> for ExampleMeasurementMethodEnumIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EXAMPLE_MEASUREMENT_METHOD_ENUM_IRI_HTTP
			|| *other == EXAMPLE_MEASUREMENT_METHOD_ENUM_IRI_HTTPS
	}
}
impl PartialEq<ExampleMeasurementMethodEnumIri> for &str {
	fn eq(&self, other: &ExampleMeasurementMethodEnumIri) -> bool {
		*self == EXAMPLE_MEASUREMENT_METHOD_ENUM_IRI_HTTP
			|| *self == EXAMPLE_MEASUREMENT_METHOD_ENUM_IRI_HTTPS
	}
}
pub struct ExampleMeasurementMethodEnumIriOrLabel;
impl PartialEq<&str> for ExampleMeasurementMethodEnumIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ExampleMeasurementMethodEnumIri || *other == EXAMPLE_MEASUREMENT_METHOD_ENUM_LABEL
	}
}
impl PartialEq<ExampleMeasurementMethodEnumIriOrLabel> for &str {
	fn eq(&self, other: &ExampleMeasurementMethodEnumIriOrLabel) -> bool {
		*self == ExampleMeasurementMethodEnumIri || *self == EXAMPLE_MEASUREMENT_METHOD_ENUM_LABEL
	}
}
