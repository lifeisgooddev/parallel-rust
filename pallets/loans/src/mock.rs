#![cfg(test)]

use super::*;

use frame_support::{construct_runtime, ord_parameter_types, parameter_types, transactional};

use orml_traits::{parameter_type_with_key};
use primitives::{Amount, Balance, CurrencyId};
// use sp_runtime::{traits::AccountIdConversion, ModuleId, RuntimeDebug};
use sp_core::H256;
use sp_runtime::{testing::Header, traits::IdentityLookup, ModuleId};
use sp_std::vec::Vec;

// pub use module::*;

pub type AccountId = u128;
pub type BlockNumber = u64;

pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;

pub const NATIVE: CurrencyId = CurrencyId::Native;
pub const USDC: CurrencyId = CurrencyId::USDC;
pub const DOT: CurrencyId = CurrencyId::DOT;
pub const BTC: CurrencyId = CurrencyId::BTC;
pub const KSM: CurrencyId = CurrencyId::KSM;

mod loans {
    pub use super::super::*;
}

parameter_types! {
    pub const GetNativeCurrencyId: CurrencyId = NATIVE;

}

parameter_types! {
    pub const BlockHashCount: u64 = 250;
}

impl frame_system::Config for Runtime {
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = ::sp_runtime::traits::BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type BlockWeights = ();
    type BlockLength = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type DbWeight = ();
    type BaseCallFilter = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
}

parameter_type_with_key! {
    pub ExistentialDeposits: |currency_id: CurrencyId| -> Balance {
        Default::default()
    };
}

impl orml_tokens::Config for Runtime {
    type Event = Event;
    type Balance = Balance;
    type Amount = Amount;
    type CurrencyId = CurrencyId;
    type WeightInfo = ();
    type ExistentialDeposits = ExistentialDeposits;
    type OnDust = ();
}

impl orml_currencies::Config for Runtime {
    type Event = Event;
    type MultiCurrency = Tokens;
    type NativeCurrency =
        orml_currencies::BasicCurrencyAdapter<Runtime, Balances, Amount, BlockNumber>;
    type GetNativeCurrencyId = GetNativeCurrencyId;
    type WeightInfo = ();
}

parameter_types! {
    pub const ExistentialDeposit: Balance = 1;
}

impl pallet_balances::Config for Runtime {
    type Balance = Balance;
    type DustRemoval = ();
    type Event = Event;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = frame_system::Module<Runtime>;
    type MaxLocks = ();
    type WeightInfo = ();
}

impl Config for Runtime {
    type Event = Event;
    type Currency = Currencies;
    type ModuleId = LoansModuleId;
}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
type Block = frame_system::mocking::MockBlock<Runtime>;

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Module, Call, Storage, Config, Event<T>},
        Tokens: orml_tokens::{Module, Storage, Event<T>, Config<T>},
        Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>},
        Currencies: orml_currencies::{Module, Call, Event<T>},
        Loans: loans::{Module, Storage, Call, Config, Event<T>},
    }
);

parameter_types! {
    pub const LoansModuleId: ModuleId = ModuleId(*b"par/loan");
}

pub struct ExtBuilder {
    endowed_accounts: Vec<(AccountId, CurrencyId, Balance)>,
}

impl Default for ExtBuilder {
    fn default() -> Self {
        Self {
            endowed_accounts: vec![
                (ALICE, DOT, 1000),
                (ALICE, BTC, 1000),
                (BOB, DOT, 1000),
                (BOB, BTC, 1000),
            ],
        }
    }
}

impl ExtBuilder {
    pub fn build(self) -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Runtime>()
            .unwrap();

        orml_tokens::GenesisConfig::<Runtime> {
            endowed_accounts: self.endowed_accounts,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        loans::GenesisConfig {
            currencies: vec![
                CurrencyId::DOT,
                CurrencyId::KSM,
                CurrencyId::BTC,
                CurrencyId::USDC,
            ],
            total_supply: 100 * 10u128.pow(18),    // 100
            total_borrows: 50 * 10u128.pow(18),    // 50
            borrow_index: 10u128.pow(18),          // 1
            exchange_rate: 2 * 10u128.pow(16),     // 0.02
            base_rate: 2 * 10u128.pow(16),         // 0.02
            multiplier_per_year: 10u128.pow(17),   // 0.1
            jump_muiltiplier: 11 * 10u128.pow(17), // 1.1
            kink: 8 * 10u128.pow(17),              // 0.8
            collateral_rate: {
                let mut res = vec![];
                res.push((CurrencyId::DOT, 5 * 10u128.pow(17))); // 0.5
                res.push((CurrencyId::KSM, 5 * 10u128.pow(17))); // 0.5
                res.push((CurrencyId::BTC, 5 * 10u128.pow(17))); // 0.5
                res.push((CurrencyId::USDC, 5 * 10u128.pow(17))); // 0.5
                res
            },
        }
        .assimilate_storage::<Runtime>(&mut t)
        .unwrap();

        // t.into()
        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}
