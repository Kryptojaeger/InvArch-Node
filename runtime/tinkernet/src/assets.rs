use crate::{
    common_types::AssetId, constants::TreasuryAccount, AccountId, Balance, Balances, BlockNumber,
    Event, ExistentialDeposit, MaxLocks, MaxReserves, Origin, Runtime, Tokens,
};
use codec::{Decode, Encode};
use frame_support::{
    parameter_types,
    traits::{Contains, EnsureOrigin, EnsureOriginWithArg},
};
use frame_system::EnsureRoot;
use orml_asset_registry::ExistentialDeposits as AssetRegistryExistentialDeposits;
use orml_currencies::BasicCurrencyAdapter;
use orml_traits::parameter_type_with_key;
use scale_info::TypeInfo;

pub const CORE_ASSET_ID: AssetId = 0;

parameter_types! {
    pub const NativeAssetId: AssetId = CORE_ASSET_ID;
}

pub struct AssetAuthority;
impl EnsureOriginWithArg<Origin, Option<u32>> for AssetAuthority {
    type Success = ();

    fn try_origin(origin: Origin, _asset_id: &Option<u32>) -> Result<Self::Success, Origin> {
        EnsureRoot::try_origin(origin)
    }
}

#[derive(Debug, TypeInfo, Encode, Decode, PartialEq, Eq, Clone)]
pub struct CustomAssetMetadata {
    pub fee_per_second: Option<u128>,
}

impl orml_asset_registry::Config for Runtime {
    type Event = Event;
    type AuthorityOrigin = AssetAuthority;
    type AssetId = AssetId;
    type Balance = Balance;
    type AssetProcessor = orml_asset_registry::SequentialId<Runtime>;
    type CustomMetadata = CustomAssetMetadata;
    type WeightInfo = ();
}

pub struct DustRemovalWhitelist;
impl Contains<AccountId> for DustRemovalWhitelist {
    fn contains(a: &AccountId) -> bool {
        // Always whitelists treasury account
        *a == TreasuryAccount::get()
    }
}

pub type Amount = i128;

parameter_type_with_key! {
      pub ExistentialDeposits: |currency_id: AssetId| -> Balance {
          if currency_id == &CORE_ASSET_ID {
              ExistentialDeposit::get()
          } else {
           AssetRegistryExistentialDeposits::<Runtime>::get(currency_id)
          }
      };
}

impl orml_tokens::Config for Runtime {
    type Event = Event;
    type Balance = Balance;
    type Amount = Amount;
    type CurrencyId = AssetId;
    type WeightInfo = ();
    type ExistentialDeposits = ExistentialDeposits;
    type OnDust = orml_tokens::TransferDust<Runtime, TreasuryAccount>;
    type MaxLocks = MaxLocks;
    type DustRemovalWhitelist = DustRemovalWhitelist;
    type OnNewTokenAccount = ();
    type OnKilledTokenAccount = ();
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = [u8; 8];
}

impl orml_currencies::Config for Runtime {
    type MultiCurrency = Tokens;
    type NativeCurrency = BasicCurrencyAdapter<Runtime, Balances, Amount, BlockNumber>;
    type GetNativeCurrencyId = NativeAssetId;
    type WeightInfo = ();
}
