# Substrate Differential Pallet

This is a [Substrate](https://github.com/paritytech/substrate) pallet that allows to configure the distribution
of the block weight between the pallets. The configuration is done while the runtime is being constructed.
The extrinsic filtering logic is implemented in the corresponding `SignedExtension`.

## Usage

- Add the dependency to your `Cargo.toml`:

```toml
substrate-differential = { version = "0.1", default-features = false, git = "https://github.com/kalaninja/substrate-differential.git" }
```

- Add the pallet to your runtime:

```rust
parameter_types! {
    pub PalletsWeightDistribution: substrate_differential::limits::PalletsWeightDistribution =
        substrate_differential::limits::PalletsWeightDistribution::build_with::<PalletInfo>()
            .add::<Balances>(Perbill::from_percent(30))
            .add::<Sudo>(Perbill::from_percent(10))
            .build_or_panic();
}

impl substrate_differential::Config for Runtime {
    type RuntimeCall = RuntimeCall;
    type PalletsWeightDistribution = PalletsWeightDistribution;
}

construct_runtime!(
    pub struct Runtime
    where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        // ...
        Differential: substrate_differential,
    }
);
```

- Add the `DifferentiatePallets` extension in the `SignedExtra` checklist:

```rust
pub type SignedExtra = (
    // ...
    substrate_differential::DifferentiatePallets<Runtime>,
);
```

- `cargo build --release`

## Sample

The example usage is available in
the [substrate-differential-sample](https://github.com/kalaninja/substrate-differential-sample) repository.

## Disclaimer

This code not audited and reviewed for production use cases. You can expect bugs and security vulnerabilities. Do not
use it as-is in real applications.