use std::time::Duration;
use tests::test_component::TestRunner;
use tests::windows::foundation::{IPropertyValue, PropertyValue};
use windows::{Interface, TimeSpan};

#[test]
fn conversion() -> windows::Result<()> {
    let a: TimeSpan = Duration::from_millis(1234).into();
    let b = TestRunner::create_time_span(1234)?;
    assert_eq!(a, b);

    let c: Duration = b.into();
    assert_eq!(c.as_millis(), 1234);

    Ok(())
}

#[test]
fn duration_param() -> windows::Result<()> {
    let object = PropertyValue::create_time_span(Duration::from_millis(1234))?;
    let pv: IPropertyValue = object.cast()?;
    assert!(pv.get_time_span()? == Duration::from_millis(1234).into());

    Ok(())
}

#[test]
fn time_span_param() -> windows::Result<()> {
    let object = PropertyValue::create_time_span(TestRunner::create_time_span(1234)?)?;
    let pv: IPropertyValue = object.cast()?;
    assert!(pv.get_time_span()? == Duration::from_millis(1234).into());

    Ok(())
}
