module wallet_staking::simple_app {
    use sui::coin::{Self, Coin};
    use sui::sui::SUI;
    use sui::balance::{Self, Balance};
    use sui::object::{Self, UID};
    use sui::transfer;
    use sui::tx_context::{Self, TxContext};

    /// Struct representing a simple User Wallet holding SUI.
    struct Wallet has key {
        id: UID,
        balance: Balance<SUI>
    }

    /// Struct representing a Staking Pool.
    struct StakingPool has key {
        id: UID,
        staked_balance: Balance<SUI>
    }

    /// A configuration struct to test abilities (store, drop, copy).
    struct Config has store, drop, copy {
        dummy_field: u64
    }

    /// create a new empty wallet for the user
    public entry fun create_wallet(ctx: &mut TxContext) {
        let wallet = Wallet {
            id: object::new(ctx),
            balance: balance::zero()
        };
        // Internal call to test 'Calls' edge
        helper_init(&mut wallet);
        transfer::transfer(wallet, tx_context::sender(ctx));
    }

    /// Helper function to test private visibility and internal calls
    fun helper_init(_wallet: &mut Wallet) {
        // Do nothing, just for testing call graph
    }

    /// Deposit coins into the wallet
    public entry fun deposit(wallet: &mut Wallet, coin: Coin<SUI>) {
        let balance = coin::into_balance(coin);
        balance::join(&mut wallet.balance, balance);
    }

    /// Withdraw coins from the wallet
    public entry fun withdraw(wallet: &mut Wallet, amount: u64, ctx: &mut TxContext) {
        let split_balance = balance::split(&mut wallet.balance, amount);
        let coin = coin::from_balance(split_balance, ctx);
        transfer::public_transfer(coin, tx_context::sender(ctx));
    }

    // --- Staking Pool Logic ---

    /// Create a shared Staking Pool
    public fun create_staking_pool(ctx: &mut TxContext) {
        let pool = StakingPool {
            id: object::new(ctx),
            staked_balance: balance::zero()
        };
        transfer::share_object(pool);
    }

    /// Stake SUI from a coin into the pool
    public entry fun stake(pool: &mut StakingPool, coin: Coin<SUI>) {
        let balance = coin::into_balance(coin);
        balance::join(&mut pool.staked_balance, balance);
    }

    /// Unstake SUI from the pool
    public entry fun unstake(pool: &mut StakingPool, amount: u64, ctx: &mut TxContext) {
        let split_balance = balance::split(&mut pool.staked_balance, amount);
        let coin = coin::from_balance(split_balance, ctx);
        transfer::public_transfer(coin, tx_context::sender(ctx));
    }

    #[test_only]
    public fun init_for_testing(ctx: &mut TxContext) {
        create_staking_pool(ctx);
    }
}
