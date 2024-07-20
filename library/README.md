# <img height="50" alt="ephemeris" src="https://github.com/user-attachments/assets/3c9283a8-b4ac-4ba3-bc54-7f089d126adf"/>

Ephemeris enables the creation short-lived program derived addresses (PDAs) signers. These signers are used to create accounts which can be "safely" closed since the same account address (PDA signer) cannot be recreated after a time period, measured in terms of slots.

This feature is useful to avoid reusing an account for something completely different or in cases when applications or indexers store any information about the account, which could get out of sync if the account is closed and recreated on the same address.

## Getting Started

From your project folder:

```bash
cargo add ephemeris
```

On your program, you replace the use of `system_instruction::create_account` with `ephemeris::create_account`:
```rust
use ephemeris::{Arguments, create_account};

create_account(
  program_id,
  Arguments {
    to: ctx.accounts.to,
    from: ctx.accounts.from,
    base: None,
    slot,
    lamports,
    space,
    owner: Some(system_program::ID),
  },
)?;
```
The arguments for the `create_account` are as follows:
* `program_id`:
   It is the address of your program (the account derivation will be done
   within the scope of the program).

* `to` (signer, writable):
  It is the funding account.

* `from` (writable):
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
  Optinal program that will own the new account (it default to `program_id` if
  omitted).

### 💡 Important

`create_account` uses the default `TTL` value of `150` slots. This is typically the number of slots that a `blockhash` is available and maximizes the chance of the account creation to succeed. You can use the `create_account_with_ttl` if you want to use a different `TTL` value – a lower `TTL` provides a shorter interval for the PDA signer to be available. At the same time, if your transaction is not executed within the `TTL` slots, it will fail.

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
