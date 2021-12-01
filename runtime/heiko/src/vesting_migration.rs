use frame_support::{
    log,
    traits::{Currency, Get},
    weights::Weight,
    BoundedVec,
};
use sp_runtime::traits::CheckedDiv;
use sp_std::convert::TryFrom;
pub(crate) type BalanceOf<T> = <<T as orml_vesting::Config>::Currency as Currency<
    <T as frame_system::Config>::AccountId,
>>::Balance;
pub(crate) type VestingScheduleOf<T> =
    orml_vesting::VestingSchedule<<T as frame_system::Config>::BlockNumber, BalanceOf<T>>;

const OLD_START: u32 = 231619;
const OLD_PERIOD_COUNT: u32 = 2628000;

const NEW_START: u32 = 0;
const NEW_PERIOD: u32 = 1;
const NEW_PERIOD_COUNT: u32 = 2160000;

const TOTAL_ACCOUNTS: u64 = 10680;
const ACCOUNT_CONTAIN_ONE_SCHEDULE: u64 = 10628;
const ACCOUNT_CONTAIN_TWO_SCHEDULE: u64 = 52;
const ACCOUNT_CONTAIN_MORE_SCHEDULE: u64 = 0;
const TOTAL_UPDATE_SCHEDULES: u64 = 10731;
const TOTAL_AMOUNT_BEFORE_AMOUNT: u128 = 31450977794836396000;
const TOTAL_AMOUNT_AFTER_AMOUNT: u128 = 31450977784297360000;

pub fn migrate<T: frame_system::Config + orml_vesting::Config>() -> Weight {
    orml_vesting::VestingSchedules::<T>::translate_values::<
        BoundedVec<VestingScheduleOf<T>, T::MaxVestingSchedules>,
        _,
    >(|v| {
        let mut new_v = BoundedVec::default();
        v.iter().for_each(|vesting_schedule| {
            let _ = new_v.try_push(update_schedule::<T>(vesting_schedule));
        });
        Some(new_v)
    });
    <T as frame_system::Config>::BlockWeights::get().max_block
}

fn update_schedule<T: frame_system::Config + orml_vesting::Config>(
    vesting_schedule: &VestingScheduleOf<T>,
) -> VestingScheduleOf<T> {
    if !vesting_schedule.start.eq(&OLD_START.into())
        || !vesting_schedule.period_count.eq(&OLD_PERIOD_COUNT)
    {
        log::warn!(target: "runtime::orml_vesting", "no need update vesting_schedule: {:?}", vesting_schedule);
        return vesting_schedule.clone();
    }

    vesting_schedule
        .total_amount()
        .and_then(|total| total.checked_div(&NEW_PERIOD_COUNT.into()))
        .and_then(|per_period| {
            Some(VestingScheduleOf::<T> {
                start: NEW_START.into(),
                period: NEW_PERIOD.into(),
                period_count: NEW_PERIOD_COUNT,
                per_period,
            })
        })
        .unwrap_or(vesting_schedule.clone())
}

/// Some checks prior to migration. This can be linked to
/// [`frame_support::traits::OnRuntimeUpgrade::pre_upgrade`] for further testing.
///
/// Panics if anything goes wrong.
pub fn pre_migrate<T: frame_system::Config + orml_vesting::Config>()
where
    u128: From<BalanceOf<T>>,
{
    let mut count_total = 0u64;
    let mut count_one = 0u64;
    let mut count_two = 0u64;
    let mut count_more = 0u64;
    let mut count_need_update = 0u64;
    let mut total_amount: BalanceOf<T> = 0u32.into();
    orml_vesting::VestingSchedules::<T>::iter().for_each(|(_k, v)| {
        count_total += 1;
        let length = v.len();
        if length == 1 {
            count_one += 1;
        } else if length == 2 {
            count_two += 1;
        } else if length > 2 {
            count_more += 1;
        }
        v.iter().for_each(|s| {
            if s.start.eq(&OLD_START.into()) && s.period_count.eq(&OLD_PERIOD_COUNT) {
                count_need_update += 1;
            }
            total_amount += s.per_period * s.period_count.into();
        });
    });

    log::info!(
        target: "runtime::orml_vesting",
        "{}, total accounts: {}, one schedule: {}, two schedule: {}, more schedule: {}, schedule need update: {}, total_amount: {:?}",
        "pre-migration", count_total, count_one, count_two, count_more, count_need_update,total_amount
    );
    assert_eq!(count_total, TOTAL_ACCOUNTS);
    assert_eq!(count_one, ACCOUNT_CONTAIN_ONE_SCHEDULE);
    assert_eq!(count_two, ACCOUNT_CONTAIN_TWO_SCHEDULE);
    assert_eq!(count_more, ACCOUNT_CONTAIN_MORE_SCHEDULE);
    assert_eq!(count_need_update, TOTAL_UPDATE_SCHEDULES);
    assert_eq!(
        u128::try_from(total_amount).unwrap(),
        TOTAL_AMOUNT_BEFORE_AMOUNT
    );
}

/// Some checks for after migration. This can be linked to
/// [`frame_support::traits::OnRuntimeUpgrade::post_upgrade`] for further testing.
///
/// Panics if anything goes wrong.
pub fn post_migrate<T: frame_system::Config + orml_vesting::Config>()
where
    u128: From<BalanceOf<T>>,
{
    let mut count_total = 0u64;
    let mut count_one = 0u64;
    let mut count_two = 0u64;
    let mut count_more = 0u64;
    let mut count_success_update = 0u64;
    let mut total_amount: BalanceOf<T> = 0u32.into();
    orml_vesting::VestingSchedules::<T>::iter().for_each(|(_k, v)| {
        count_total += 1;
        let length = v.len();
        if length == 1 {
            count_one += 1;
        } else if length == 2 {
            count_two += 1;
        } else if length > 2 {
            count_more += 1;
        }
        v.iter().for_each(|s| {
            if s.start.eq(&NEW_START.into()) && s.period_count.eq(&NEW_PERIOD_COUNT) {
                count_success_update += 1;
            }
            total_amount += s.per_period * s.period_count.into();
        });
    });

    log::info!(
        target: "runtime::orml_vesting",
        "{}, total accounts: {}, one schedule: {}, two schedule: {}, more schedule: {}, schedule success update: {}, total_amount: {:?}",
        "post-migration", count_total, count_one, count_two, count_more, count_success_update, total_amount
    );

    assert_eq!(count_total, TOTAL_ACCOUNTS);
    assert_eq!(count_one, ACCOUNT_CONTAIN_ONE_SCHEDULE);
    assert_eq!(count_two, ACCOUNT_CONTAIN_TWO_SCHEDULE);
    assert_eq!(count_more, ACCOUNT_CONTAIN_MORE_SCHEDULE);
    assert_eq!(count_success_update, TOTAL_UPDATE_SCHEDULES);
    assert_eq!(
        u128::try_from(total_amount).unwrap(),
        TOTAL_AMOUNT_AFTER_AMOUNT
    );
}
