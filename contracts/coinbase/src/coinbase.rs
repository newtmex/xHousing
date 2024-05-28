#![no_std]

pub mod coinbase_proxy;
mod x_housing_interactions;
pub mod x_project_funding_interactions;

#[allow(unused_imports)]
use multiversx_sc::imports::*;

use multiversx_sc::storage_set;
use xht::{self, XHTTrait, XHT};

/// The `Coinbase` contract is responsible for managing the distribution and economics of XHT tokens
/// within the xHousing platform. It interacts with the `XHTModule` to perform token operations
/// such as minting and burning. The contract ensures efficient token supply management by
/// distributing XHT tokens to users participating in platform activities (e.g., fundraising,
/// staking, referrals) and burning tokens to support deflationary mechanisms. Ownership and
/// access control mechanisms are included to restrict certain operations to authorized parties,
/// maintaining the integrity and security of the platform's token economy.
#[multiversx_sc::contract]
pub trait Coinbase:
    xht::XHTModule
    + x_housing_interactions::XHousingModule
    + x_project_funding_interactions::XProjectFundingInteraction
{
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[only_owner]
    #[endpoint]
    #[payable("EGLD")]
    fn register_xht(&self) {
        require!(self.xht().is_empty(), "XHT already minted");

        let gas_left = self.blockchain().get_gas_left();
        let gas_limit = 60_000_000;
        let extra_gas = 10_000_000;
        require!(
            gas_left >= (gas_limit + extra_gas),
            "Insufficient gas to register XHT"
        );

        let issue_cost = self.call_value().egld_value().clone_value();
        let token_display_name = ManagedBuffer::from("XHousingToken");
        let token_ticker = ManagedBuffer::from("XHT");

        storage_set(
            self.xht().get_storage_key(),
            &TokenMapperState::<Self::Api>::Pending,
        );

        Tx::new_tx_from_sc()
            .to(ESDTSystemSCAddress)
            .typed(ESDTSystemSCProxy)
            .issue_fungible(
                issue_cost,
                &token_display_name,
                &token_ticker,
                &XHT::max_supply(),
                FungibleTokenProperties {
                    num_decimals: XHT::<Self::Api>::decimals() as usize,
                    can_burn: true,
                    can_freeze: false,
                    can_wipe: false,
                    can_pause: false,
                    can_mint: false,
                    can_change_owner: false,
                    can_upgrade: false,
                    can_add_special_roles: false,
                },
            )
            .callback(self.callbacks().register_xht_cb())
            .async_call_and_exit()
    }

    #[callback]
    fn register_xht_cb(&self, #[call_result] result: ManagedAsyncCallResult<()>) {
        let (token_id, returned_tokens) = self.call_value().egld_or_single_fungible_esdt();

        match result {
            ManagedAsyncCallResult::Ok(()) => self.xht().set_token_id(token_id.unwrap_esdt()),
            ManagedAsyncCallResult::Err(_) => {
                self.xht().clear();

                if token_id.is_egld() && returned_tokens > 0u64 {
                    self.send()
                        .direct_egld(&self.blockchain().get_owner_address(), &returned_tokens);
                }
            }
        }
    }
}
