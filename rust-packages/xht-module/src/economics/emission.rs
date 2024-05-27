use core::ops::{Div, Mul};

use multiversx_sc::{
    api::{ErrorApi, ErrorApiImpl, ManagedTypeApi},
    types::{BigFloat, BigUint},
};
use utils::types::Epoch;

use crate::{XHTTrait, XHT};

impl<M: ManagedTypeApi> EmissionTrait<M> for XHT<M> {}

pub trait EmissionTrait<M: ManagedTypeApi> {
    fn decay_rate() -> BigFloat<M> {
        BigFloat::from_sci(999_8, -4)
    }

    fn ln_decay_rate() -> BigFloat<M> {
        BigFloat::from_sci(200_020_002_667_044, -18)
    }

    fn epoch_zero_emission() -> XHT<M> {
        XHT::from_parts(2_729, 727_036_845_720_118_116)
    }

    fn mul_by_big_float(self, rhs: BigFloat<M>) -> XHT<M>
    where
        Self: Mul<BigUint<M>, Output = BigUint<M>> + Sized,
    {
        let rhs = rhs
            .to_fixed_point(&XHT::one().into())
            .into_big_uint()
            .unwrap_or_sc_panic("decay value compute error");

        self * rhs / XHT::one()
    }

    fn div_by_big_float(self, rhs: BigFloat<M>) -> XHT<M>
    where
        Self: Mul<BigUint<M>, Output = BigUint<M>> + Div<BigUint<M>, Output = BigUint<M>> + Sized,
    {
        let rhs = rhs
            .to_fixed_point(&XHT::one().into())
            .into_big_uint()
            .unwrap_or_sc_panic("decay value compute error");

        self * XHT::one() / rhs
    }

    fn emission_at_epoch(epoch: Epoch) -> XHT<M> {
        if epoch == 0 {
            return Self::epoch_zero_emission();
        }

        Self::emission_through_epoch_range(epoch - 1, epoch)
    }

    /// Computes the amount of XHTs to emit after `epoch_start` upto `epoch_end`,
    /// the amount emited at `epoch_start` is excluded
    fn emission_through_epoch_range(epoch_start: Epoch, epoch_end: Epoch) -> XHT<M> {
        let epoch_start = epoch_to_i32::<M>(epoch_start);
        let epoch_end = epoch_to_i32::<M>(epoch_end);

        if epoch_end <= epoch_start {
            M::error_api_impl().signal_error(b"Invalid epoch end");
        }

        let decay_value =
            (Self::decay_rate().pow(epoch_end) - Self::decay_rate().pow(epoch_start)).magnitude();

        XHT::epoch_zero_emission()
            .div_by_big_float(Self::ln_decay_rate())
            .mul_by_big_float(decay_value)
    }
}

fn epoch_to_i32<A: ErrorApi>(epoch: Epoch) -> i32 {
    i32::try_from(epoch)
        .unwrap_or_else(|_| A::error_api_impl().signal_error(b"epoch value out of range"))
}

#[cfg(test)]
mod tests {
    extern crate std;

    use core::ops::Add;

    use multiversx_sc::types::BigUint;
    use multiversx_sc_scenario::api::StaticApi;

    use super::EmissionTrait;
    use crate::XHTTrait;

    type Xht = super::XHT<StaticApi>;

    #[test]
    fn check_emission_schedule() {
        let ecosystem_distibution_funds = Xht::ecosystem_distibution_funds();
        let computed_total_emitted = Xht::epoch_zero_emission()
            .div_by_big_float(Xht::ln_decay_rate())
            .add(Xht::epoch_zero_emission());

        assert_eq!(
            ecosystem_distibution_funds, computed_total_emitted,
            "Ecosystem distribution funds must relate properly with natural log of decay rate"
        );

        let ln_decay_rate_big_unint = Xht::ln_decay_rate()
            .to_fixed_point(&Xht::one().into())
            .into_big_uint()
            .unwrap_or_sc_panic("");
        let decay_rate = Xht::decay_rate()
            .to_fixed_point(&Xht::one().into())
            .into_big_uint()
            .unwrap_or_sc_panic("")
            .to_u64()
            .unwrap() as f64
            / Xht::one().to_u64().unwrap() as f64;
        assert_eq!(
            ln_decay_rate_big_unint,
            BigUint::from((decay_rate.ln().abs() * Xht::one().to_u64().unwrap() as f64) as u64),
            "Decay rate must be accurate"
        );

        assert_eq!(
            Xht::emission_at_epoch(0),
            Xht::epoch_zero_emission(),
            "Literal and computed XHT emission at epoch zero must be the same"
        );

        let epoch_start = 3_650;
        let epoch_end = (epoch_start + 1) + 100;

        let mut cumulative = BigUint::zero();
        // Nine decimal place precission
        let one = 1_000_000_000u64;
        // The first epoch is skiped here, due to the way `emission_through_epoch_range` computes
        for epoch in (epoch_start + 1)..=epoch_end {
            cumulative += Xht::emission_at_epoch(epoch);

            let ratio =
                &(Xht::emission_through_epoch_range(epoch_start, epoch) * one) / &cumulative;
            assert_eq!(ratio, one);
        }

        let computed = Xht::emission_through_epoch_range(epoch_start, epoch_end);

        assert_eq!(
            computed * one / cumulative,
            one,
            "computed emission through epochs should be same as the cumulative one"
        );

        assert_eq!(
            Xht::ecosystem_distibution_funds(),
            Xht::emission_through_epoch_range(0, 1_000_000).add(Xht::epoch_zero_emission()),
            "All ecossytem distribution funds should be distributed"
        )
    }

    #[test]
    #[should_panic(expected = "Invalid epoch end")]
    fn emission_through_epoch_range() {
        Xht::emission_through_epoch_range(3, 1);
    }
}
