use core::ops::{Div, Mul};

use multiversx_sc::api::{ErrorApi, ErrorApiImpl};

pub fn percent_share_factory<'a, Api: ErrorApi, Amt: 'a>(
    amt: &'a Amt,
    max_percent: u32,
) -> impl FnMut(u32) -> Amt + 'a
where
    &'a Amt: Mul<u32, Output = Amt>,
    Amt: Div<u32, Output = Amt>,
{
    let mut total_percentage_used = 0;

    move |percentage| {
        total_percentage_used += percentage;
        if total_percentage_used > max_percent {
            Api::error_api_impl().signal_error(b"cannot share more than 100%")
        }

        amt * percentage / max_percent
    }
}

#[cfg(test)]
mod helpers_tests {
    use multiversx_sc_scenario::api::SingleTxApi;

    use super::*;

    #[test]
    fn percent_share_factory_test() {
        let value = 300;

        let mut percent_share = percent_share_factory::<SingleTxApi, u32>(&value, 100);

        let share_1 = percent_share(50);
        let share_2 = percent_share(50);

        assert_eq!(share_1 + share_2, 300);
    }

    #[test]
    #[should_panic(expected = "cannot share more than 100%")]
    fn percent_share_factory_test_over_100() {
        let value = 25;
        let mut percent_share = percent_share_factory::<SingleTxApi, u32>(&value, 100);

        percent_share(1);
        // this should pannic
        percent_share(100);
    }
}
