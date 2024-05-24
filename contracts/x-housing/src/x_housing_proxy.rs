// Code generated by the multiversx-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use multiversx_sc::proxy_imports::*;

pub struct XHousingProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for XHousingProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = XHousingProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        XHousingProxyMethods { wrapped_tx: tx }
    }
}

pub struct XHousingProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> XHousingProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init(
        self,
    ) -> TxTypedDeploy<Env, From, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_deploy()
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> XHousingProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn upgrade(
        self,
    ) -> TxTypedUpgrade<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_upgrade()
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> XHousingProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    /// Anyone can call this endpoint to register their wallet address as users of the xHousing platform 
    /// so they can get a referral ID that they can use to leverage other earning opportunities on the platform 
    pub fn create_ref_id<
        Arg0: ProxyArg<OptionalValue<usize>>,
    >(
        self,
        referrer_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, usize> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("createRefID")
            .argument(&referrer_id)
            .original_result()
    }

    pub fn get_affiliate_details<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        user_addr: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, (usize, Option<(usize, ManagedAddress<Env::Api>)>)> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getAffiliateDetails")
            .argument(&user_addr)
            .original_result()
    }
}
