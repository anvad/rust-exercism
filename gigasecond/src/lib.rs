// use time::Duration;
use time::ext::NumericalDuration;
use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    // // option 1
    // start.saturating_add(Duration::new(1_000_000_000, 0))

    // option 2
    start + 1e9.seconds()
}
