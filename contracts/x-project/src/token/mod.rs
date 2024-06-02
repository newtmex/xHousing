use multiversx_sc_modules::default_issue_callbacks;
use utils::xpt_attributes::XPTokenAttributes;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait XPTokenModule:
    xht::XHTModule + default_issue_callbacks::DefaultIssueCallbacksModule
{
    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(registerXPToken)]
    fn register_xp_token(
        &self,
        name: ManagedBuffer,
        amount_raised: BigUint,
        xht_id: TokenIdentifier<Self::Api>,
    ) {
        require!(self.xp_token().is_empty(), "XPToken already set");
        self.xht().set_token_id(xht_id);

        let payment_amount = self.call_value().egld_value();
        self.xp_amount_raised().set(amount_raised);

        self.xp_token().issue_and_set_all_roles(
            EsdtTokenType::SemiFungible,
            payment_amount.clone_value(),
            name,
            b"XPT".into(),
            18,
            None,
        );
    }

    #[only_owner]
    #[endpoint]
    fn mint_xp_token(&self, deposit_amount: BigUint, depositor: ManagedAddress) {
        let total_deposits = self.xp_amount_raised().get();
        let max_shares = self.xp_token_max_supply().get();

        require!(total_deposits > 0, "No deposits recorded");

        let mint_share = deposit_amount * max_shares / total_deposits;
        require!(
            mint_share > BigUint::zero(),
            "Computed token shares is invalid"
        );

        let total_supply = self.xp_token_supply();
        total_supply.update(|x| *x += &mint_share);
        require!(
            total_supply.get() <= self.xp_token_max_supply().get(),
            "max supply exceeded"
        );

        let token_id = self.xp_token().get_token_id();
        let token_nonce = self.send().esdt_nft_create_compact(
            &token_id,
            &mint_share,
            &XPTokenAttributes::new(mint_share.clone(), depositor),
        );

        self.tx()
            .to(ToCaller)
            .esdt((token_id, token_nonce, mint_share))
            .transfer();
    }

    #[view(getXPTokenId)]
    #[storage_mapper("xp_token")]
    fn xp_token(&self) -> NonFungibleTokenMapper;

    #[view(getXPTokenSupply)]
    #[storage_mapper("xp_token_supply")]
    fn xp_token_supply(&self) -> SingleValueMapper<BigUint>;

    #[view(getXPTokenMaxSupply)]
    #[storage_mapper("xp_token_max_supply")]
    fn xp_token_max_supply(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("amount_raised")]
    fn xp_amount_raised(&self) -> SingleValueMapper<BigUint>;
}
