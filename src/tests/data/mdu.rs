use crate::data::mdu::{MDUError, MDU};

#[test]
fn should_match() {
    assert_eq!(
        MDU::from_work_schedule("MDU_ZAP00029"),
        Ok(MDU {
            city_code: "ZAP".to_string(),
            number:    29,
        })
    );

    assert_eq!(
        MDU::from_work_schedule("MDU_ZAP29"),
        Ok(MDU {
            city_code: "ZAP".to_string(),
            number:    29,
        })
    );
}

#[test]
fn invalid() {
    assert_eq!(MDU::from_work_schedule("MDU_00029"), Err(MDUError::NoMatch));

    assert_eq!(
        // PosOverflow
        MDU::from_work_schedule("MDU_ZAP4294967296"),
        Err(MDUError::FailedToParseNumber)
    );
}
