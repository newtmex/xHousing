use data::DistributionStorage;

multiversx_sc::imports!();

mod data;

#[multiversx_sc::module]
pub trait DistributionModule {
    #[storage_mapper("distribution_storage")]
    fn distribution_storage(&self) -> DistributionStorage<Self::Api>;
}
