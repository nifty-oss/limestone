# <img height="50" alt="limestone" src="https://github.com/user-attachments/assets/53b09445-dba6-43c4-9bdf-f4df9ab677a3"/>

A Rust client SDK for Limestone [program](https://github.com/nifty-oss/limestone).

Limestone enables the creation short-lived program derived address (PDA) signers. These signers are used to create accounts which can be "safely" closed since the same account address signer cannot be recreated after a time period &mdash; `time` in this case measured in terms of slots.

This feature is useful to avoid reusing an account for something completely different, which can create problems for applications and off-chain services that store any information about the account. Therefore, accounts can be closed, avoiding bloating account state, returning all rent funds and enabling the use of accounts to represent "ephemeral" concepts (e.g., receipts, tickets) without incurring unnecessary costs.

> This crate is intended to be used by clients interacting with the `limestone` program. To integrate `limestone` account derivation into your PDAs, you can use the library instead.

## Getting Started

From your project folder:

```bash
cargo add limestone-client
```

On your client, replace the use of `system_instruction::create_account` with `limestone::instructions::create_account`. The SDK provides helper to build the required `Instruction`:
```rust
use limestone_client::{find_pda, instructions::CreateAccountBuilder};

// Creating an account to be used as a SPL Token 2022 mint

let (pda, _) = find_pda(&payer, slot);
let account_size = 200; // 200 bytes

let create_ix = CreateAccountBuilder::new()
    .from(payer)
    .to(pda)
    .slot(slot)
    .lamports(rent.minimum_balance(account_size))
    .space(account_size as u64)
    .owner(spl_token_2022::ID)
    .instruction();
```
The arguments required to create an account are as follows:

* `from` (signer:
  It is the funding account.

* `to`:
  It is the account to be created (must be a PDA of `[base, slot]` derived from
  program_id).

* `base` (signer, optional):
  Optional signer for the account derivation (it default to `from` if omitted).

* `slot`:
  The slot number for the derivation (the slot needs to be within the valid range,
  i.e., not older than `current slot - TTL`).

* `lamports`:
  The lamports to be transferred to the new account (must be at least the amount
  needed for the account to be rent-exempt).

* `space`:
  The data size for the new account.

* `owner`:
  Program that will own the new account.

💡 **Note:** When using the client on-chain, replace the `CreateAccountBuilder` for `CreateAccountCpiBuilder`.

## Limitation

Although the use `TTL` defines a time period where the account creation is allowed &mdash; `150` slots is approximately 1 minute 19 seconds assuming `400`ms block times &mdash; it does not guarantee that the account is not closed and recreated between that interval. Additionally, it does not prevent an account being created, closed and recreated on the same transaction.

For protocols that need such guarantee, an addional restriction needs be added when closing an account to prevent recreation. The protocol should store the `slot` value used on the account derivation and validate that the account is being closed after `slot + TTL` &mdash; this will prevent the account recreation since the `slot` value will be too old to generate a PDA signer for the same address.

## License

Copyright (c) 2024 nifty-oss maintainers

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
