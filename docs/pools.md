# Nosana Pools

## Program Information

| Info            | Description                                                                                                                         |
|-----------------|-------------------------------------------------------------------------------------------------------------------------------------|
| Type            | [Solana Program](https://docs.solana.com/developing/intro/programs#on-chain-programs)                                               |
| Source Code     | [GitHub](https://github.com/nosana-ci/nosana-programs)                                                                              |
| Build Status    | [Anchor Verified](https://www.apr.dev/program/nosPdZrfDzND1LAR28FLMDEATUPK53K8xbRBXAirevD)                                          |
| Accounts        | [`2`](#accounts)                                                                                                                    |
| Instructions    | [`4`](#instructions)                                                                                                                |
| Types           | [`1`](#types)                                                                                                                       |
| Domain          | `nosana-pools.sol`                                                                                                                  |
|  Address        | [`nosPdZrfDzND1LAR28FLMDEATUPK53K8xbRBXAirevD`](https://explorer.solana.com/address/nosPdZrfDzND1LAR28FLMDEATUPK53K8xbRBXAirevD)    |

## Instructions

A number of 4 instruction are defined in the Nosana Pools program.

To load the program with [Anchor](https://coral-xyz.github.io/anchor/ts/index.html).

```typescript
const programId = new PublicKey('nosPdZrfDzND1LAR28FLMDEATUPK53K8xbRBXAirevD');
const idl = await Program.fetchIdl(programId.toString());
const program = new Program(idl, programId);
```

### Open

Open a [PoolAccount](#pool-account) and [VaultAccount](#vault-account).

#### Account Info

The following 8 account addresses should be provided when invoking this instruction.

| Name                   | Type                                                                                    | Description                                                                                       |
|------------------------|-----------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------|
| `pool`                 | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="#3EAF7C" />       | The [PoolAccount](#pool-account) address.                                                         |
| `vault`                | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="lightgrey" />     | The [VaultAccount](#vault-account) address.                                                       |
| `beneficiary`          | <FontIcon icon="pencil" color="lightgrey" /><FontIcon icon="key" color="lightgrey" />   | The token account that will receive the emissions from the Pool.                                  |
| `authority`            | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="#3EAF7C" />       | The signing authority of the program invocation.                                                  |
| `mint`                 | <FontIcon icon="pencil" color="lightgrey" /><FontIcon icon="key" color="lightgrey" />   | The token Mint address for this instruction.                                                      |
| `systemProgram`        | <FontIcon icon="pencil" color="lightgrey" /><FontIcon icon="key" color="lightgrey" />   | The official Solana system program address. Responsible for system CPIs.                          |
| `tokenProgram`         | <FontIcon icon="pencil" color="lightgrey" /><FontIcon icon="key" color="lightgrey" />   | The official SPL Token Program address. Responsible for token CPIs.                               |
| `rent`                 | <FontIcon icon="pencil" color="lightgrey" /><FontIcon icon="key" color="lightgrey" />   | The official Solana rent address. Responsible for lamports.                                       |

#### Arguments

The following 4 arguments should also be provided when invoking this instruction.

| Name                   | Type              | Size    | Offset  | Description                                               |
|------------------------|-------------------|---------|---------|-----------------------------------------------------------|
| `emission`             | `u64`             | `8`     | `0`     | The emission rate for the pool, per second.               |
| `startTime`            | `i64`             | `16`    | `8`     | The unix time the pool opens.                             |
| `claimType`            | `u8`              | `1`     | `24`    | The [ClaimType](#claim-type) for this pool.               |
| `closeable`            | `bool`            | `1`     | `25`    | Whether the pool should be closable or not.               |


#### Solana Dispatch ID

The Solana dispatch ID for the Open Instruction
is **`e4dc9b47c7bd3c2d`**,
which can also be expressed as an 8 byte discriminator:

```json
[228,220,155,71,199,189,60,45]
```

#### Example with Anchor

To invoke the Open Instruction
with [Anchor TS](https://coral-xyz.github.io/anchor/ts/index.html).

```typescript
let tx = await program.methods
  .open(
    emission,          // type: u64
    startTime,         // type: i64
    claimType,         // type: u8
    closeable,         // type: bool
  )
  .accounts({
    pool,              // ✓ writable, ✓ signer
    vault,             // ✓ writable, 𐄂 signer
    beneficiary,       // 𐄂 writable, 𐄂 signer
    authority,         // ✓ writable, ✓ signer
    mint,              // 𐄂 writable, 𐄂 signer
    systemProgram,     // 𐄂 writable, 𐄂 signer
    tokenProgram,      // 𐄂 writable, 𐄂 signer
    rent,              // 𐄂 writable, 𐄂 signer
  })
  .signers([poolKey, authorityKey])
  .rpc();
```

### Claim Fee

Add fees from a [PoolAccount](#pool-account) with claim type [`1`](#claim-type)

#### Account Info

The following 7 account addresses should be provided when invoking this instruction.

| Name                   | Type                                                                                    | Description                                                                                       |
|------------------------|-----------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------|
| `vault`                | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="lightgrey" />     | The [VaultAccount](#vault-account) address.                                                       |
| `rewardsReflection`    | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="lightgrey" />     | The Nosana Rewards Program's [ReflectionAccount](/programs/rewards#reflection-account) address.   |
| `rewardsVault`         | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="lightgrey" />     | The Nosana Rewards Program's [VaultAccount](/programs/rewards#vault-account) address.             |
| `pool`                 | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="lightgrey" />     | The [PoolAccount](#pool-account) address.                                                         |
| `authority`            | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="#3EAF7C" />       | The signing authority of the program invocation.                                                  |
| `tokenProgram`         | <FontIcon icon="pencil" color="lightgrey" /><FontIcon icon="key" color="lightgrey" />   | The official SPL Token Program address. Responsible for token CPIs.                               |
| `rewardsProgram`       | <FontIcon icon="pencil" color="lightgrey" /><FontIcon icon="key" color="lightgrey" />   | The [Nosana Rewards](/programs/rewards) Program address.                                          |


#### Solana Dispatch ID

The Solana dispatch ID for the Claim Fee Instruction
is **`a9204f8988e84689`**,
which can also be expressed as an 8 byte discriminator:

```json
[169,32,79,137,136,232,70,137]
```

#### Example with Anchor

To invoke the Claim Fee Instruction
with [Anchor TS](https://coral-xyz.github.io/anchor/ts/index.html).

```typescript
let tx = await program.methods
  .claimFee()
  .accounts({
    vault,             // ✓ writable, 𐄂 signer
    rewardsReflection, // ✓ writable, 𐄂 signer
    rewardsVault,      // ✓ writable, 𐄂 signer
    pool,              // ✓ writable, 𐄂 signer
    authority,         // ✓ writable, ✓ signer
    tokenProgram,      // 𐄂 writable, 𐄂 signer
    rewardsProgram,    // 𐄂 writable, 𐄂 signer
  })
  .signers([authorityKey])
  .rpc();
```

### Claim Transfer

Claim emission from a [PoolAccount](#pool-account) with claim type [`0`](#claim-type)

#### Account Info

The following 5 account addresses should be provided when invoking this instruction.

| Name                   | Type                                                                                    | Description                                                                                       |
|------------------------|-----------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------|
| `vault`                | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="lightgrey" />     | The [VaultAccount](#vault-account) address.                                                       |
| `beneficiary`          | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="lightgrey" />     | The token account that will receive the emissions from the Pool.                                  |
| `pool`                 | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="lightgrey" />     | The [PoolAccount](#pool-account) address.                                                         |
| `authority`            | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="#3EAF7C" />       | The signing authority of the program invocation.                                                  |
| `tokenProgram`         | <FontIcon icon="pencil" color="lightgrey" /><FontIcon icon="key" color="lightgrey" />   | The official SPL Token Program address. Responsible for token CPIs.                               |


#### Solana Dispatch ID

The Solana dispatch ID for the Claim Transfer Instruction
is **`cab23abee6eae511`**,
which can also be expressed as an 8 byte discriminator:

```json
[202,178,58,190,230,234,229,17]
```

#### Example with Anchor

To invoke the Claim Transfer Instruction
with [Anchor TS](https://coral-xyz.github.io/anchor/ts/index.html).

```typescript
let tx = await program.methods
  .claimTransfer()
  .accounts({
    vault,             // ✓ writable, 𐄂 signer
    beneficiary,       // ✓ writable, 𐄂 signer
    pool,              // ✓ writable, 𐄂 signer
    authority,         // ✓ writable, ✓ signer
    tokenProgram,      // 𐄂 writable, 𐄂 signer
  })
  .signers([authorityKey])
  .rpc();
```

### Close

Close a [PoolAccount](#pool-account) and [VaultAccount](#vault-account).

#### Account Info

The following 5 account addresses should be provided when invoking this instruction.

| Name                   | Type                                                                                    | Description                                                                                       |
|------------------------|-----------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------|
| `vault`                | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="lightgrey" />     | The [VaultAccount](#vault-account) address.                                                       |
| `user`                 | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="lightgrey" />     | The user token account that will debit/credit the tokens.                                         |
| `pool`                 | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="lightgrey" />     | The [PoolAccount](#pool-account) address.                                                         |
| `authority`            | <FontIcon icon="pencil" color="#3EAF7C" /><FontIcon icon="key" color="#3EAF7C" />       | The signing authority of the program invocation.                                                  |
| `tokenProgram`         | <FontIcon icon="pencil" color="lightgrey" /><FontIcon icon="key" color="lightgrey" />   | The official SPL Token Program address. Responsible for token CPIs.                               |


#### Solana Dispatch ID

The Solana dispatch ID for the Close Instruction
is **`62a5c9b16c41ce60`**,
which can also be expressed as an 8 byte discriminator:

```json
[98,165,201,177,108,65,206,96]
```

#### Example with Anchor

To invoke the Close Instruction
with [Anchor TS](https://coral-xyz.github.io/anchor/ts/index.html).

```typescript
let tx = await program.methods
  .close()
  .accounts({
    vault,             // ✓ writable, 𐄂 signer
    user,              // ✓ writable, 𐄂 signer
    pool,              // ✓ writable, 𐄂 signer
    authority,         // ✓ writable, ✓ signer
    tokenProgram,      // 𐄂 writable, 𐄂 signer
  })
  .signers([authorityKey])
  .rpc();
```

## Accounts

A number of 2 accounts make up for the Nosana Pools Program's state.

### Pool Account

The `PoolAccount` struct holds all the information for any given pool.
The total size of this account is `139` bytes.

| Name                        | Type                        | Size    | Offset  | Description                                                                                       |
|-----------------------------|-----------------------------|---------|---------|---------------------------------------------------------------------------------------------------|
| `authority`                 | `publicKey`                 | `32`    | `8`     | The signing authority of the program invocation.                                                  |
| `beneficiary`               | `publicKey`                 | `32`    | `40`    | The token account that will receive the emissions from the Pool.                                  |
| `claimType`                 | `u8`                        | `1`     | `72`    | The [ClaimType](#claim-type) for this pool.                                                       |
| `claimedTokens`             | `u64`                       | `8`     | `73`    | The number of tokens that have been claimed.                                                      |
| `closeable`                 | `bool`                      | `1`     | `81`    | Whether the pool should be closable or not.                                                       |
| `emission`                  | `u64`                       | `8`     | `82`    | The emission rate for the pool, per second.                                                       |
| `startTime`                 | `i64`                       | `16`    | `90`    | The unix time the pool opens.                                                                     |
| `vault`                     | `publicKey`                 | `32`    | `106`   | The [VaultAccount](#vault-account) address.                                                       |
| `vaultBump`                 | `u8`                        | `1`     | `138`   | The bump for the [VaultAccount](#vault-account).                                                  |

#### Anchor Account Discriminator

The first 8 bytes, also known as Anchor's 8 byte discriminator, for the Pool Account
are **`74d2bb77c4c43489`**, which can also be expressed in byte array:

```json
[116,210,187,119,196,196,52,137]
```

### Vault Account

The `VaultAccount` is a regular Solana Token Account.

## Types

A number of 1 type variants are defined in the Nosana Pools Program's state.

### Claim Type


The `ClaimType` of any pool describes the way withdraw ([claim](#claim)) works.

A number of 3 variants are defined in this `enum`:
| Name                                  | Number                                |
|---------------------------------------|---------------------------------------|
| `Transfer`                            | `0`                                   |
| `AddFee`                              | `1`                                   |
| `Unknown`                             | `255`                                 |
