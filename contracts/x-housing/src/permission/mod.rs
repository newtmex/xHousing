mod permissions;

pub use permissions::Permissions;

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait PermissionsModule {
    fn set_permissions(&self, address: ManagedAddress, permissions: Permissions) {
        self.permissions(address).set(permissions);
    }

    fn require_caller_any_of(&self, permissions: Permissions) {
        let caller = self.blockchain().get_caller();
        let caller_permissions = self.permissions(caller).get();
        require!(
            caller_permissions.intersects(permissions),
            "Permission denied"
        );
    }

    fn require_caller_is_x_project(&self) {
        self.require_caller_any_of(Permissions::X_PROJECT);
    }

    #[storage_mapper("permissions")]
    fn permissions(&self, address: ManagedAddress) -> SingleValueMapper<Permissions>;
}
