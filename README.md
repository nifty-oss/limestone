<h1 align="center">
  <code>limestone</code>
</h1>
<p align="center">
  <img width="400" alt="Limestone" src="https://github.com/user-attachments/assets/a889bf96-5e3f-40de-b343-b8135dfe19a9" />
</p>
<p align="center">
  Generate short-lived unique program derive address signers.
</p>

<p align="center">
  <a href="https://github.com/nifty-oss/limestone/actions/workflows/main.yml"><img src="https://img.shields.io/github/actions/workflow/status/nifty-oss/limestone/main.yml?logo=GitHub" /></a>
  <a href="https://explorer.solana.com/address/LMSToZQenurAeAutm239hcJBCgsaPNaJhNC7nJhrtdB"><img src="https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fraw.githubusercontent.com%2Fnifty-oss%limestone%2Fmain%2Fprogram%2Fidl.json&query=%24.version&label=program&logo=data:image/svg%2bxml;base64,PHN2ZyB3aWR0aD0iMzEzIiBoZWlnaHQ9IjI4MSIgdmlld0JveD0iMCAwIDMxMyAyODEiIGZpbGw9Im5vbmUiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+CjxnIGNsaXAtcGF0aD0idXJsKCNjbGlwMF80NzZfMjQzMCkiPgo8cGF0aCBkPSJNMzExLjMxOCAyMjEuMDU3TDI1OS42NiAyNzYuNTU4QzI1OC41MzcgMjc3Ljc2NCAyNTcuMTc4IDI3OC43MjUgMjU1LjY2OSAyNzkuMzgyQzI1NC4xNTkgMjgwLjAzOSAyNTIuNTMgMjgwLjM3OCAyNTAuODg0IDI4MC4zNzdINS45OTcxOUM0LjgyODcgMjgwLjM3NyAzLjY4NTY4IDI4MC4wMzUgMi43MDg1NSAyNzkuMzkzQzEuNzMxNDMgMjc4Ljc1MSAwLjk2Mjc3MSAyNzcuODM3IDAuNDk3MDIgMjc2Ljc2NEMwLjAzMTI2OTEgMjc1LjY5IC0wLjExMTI4NiAyNzQuNTA0IDAuMDg2ODcxMiAyNzMuMzVDMC4yODUwMjggMjcyLjE5NiAwLjgxNTI2NSAyNzEuMTI2IDEuNjEyNDMgMjcwLjI3TDUzLjMwOTkgMjE0Ljc2OUM1NC40Mjk5IDIxMy41NjYgNTUuNzg0MyAyMTIuNjA3IDU3LjI4OTMgMjExLjk1QzU4Ljc5NDMgMjExLjI5MyA2MC40MTc4IDIxMC45NTMgNjIuMDU5NSAyMTAuOTVIMzA2LjkzM0MzMDguMTAxIDIxMC45NSAzMDkuMjQ0IDIxMS4yOTIgMzEwLjIyMSAyMTEuOTM0QzMxMS4xOTkgMjEyLjU3NiAzMTEuOTY3IDIxMy40OSAzMTIuNDMzIDIxNC41NjRDMzEyLjg5OSAyMTUuNjM3IDMxMy4wNDEgMjE2LjgyNCAzMTIuODQzIDIxNy45NzdDMzEyLjY0NSAyMTkuMTMxIDMxMi4xMTUgMjIwLjIwMSAzMTEuMzE4IDIyMS4wNTdaTTI1OS42NiAxMDkuMjk0QzI1OC41MzcgMTA4LjA4OCAyNTcuMTc4IDEwNy4xMjcgMjU1LjY2OSAxMDYuNDdDMjU0LjE1OSAxMDUuODEzIDI1Mi41MyAxMDUuNDc0IDI1MC44ODQgMTA1LjQ3NUg1Ljk5NzE5QzQuODI4NyAxMDUuNDc1IDMuNjg1NjggMTA1LjgxNyAyLjcwODU1IDEwNi40NTlDMS43MzE0MyAxMDcuMTAxIDAuOTYyNzcxIDEwOC4wMTUgMC40OTcwMiAxMDkuMDg4QzAuMDMxMjY5MSAxMTAuMTYyIC0wLjExMTI4NiAxMTEuMzQ4IDAuMDg2ODcxMiAxMTIuNTAyQzAuMjg1MDI4IDExMy42NTYgMC44MTUyNjUgMTE0LjcyNiAxLjYxMjQzIDExNS41ODJMNTMuMzA5OSAxNzEuMDgzQzU0LjQyOTkgMTcyLjI4NiA1NS43ODQzIDE3My4yNDUgNTcuMjg5MyAxNzMuOTAyQzU4Ljc5NDMgMTc0LjU1OSA2MC40MTc4IDE3NC44OTkgNjIuMDU5NSAxNzQuOTAySDMwNi45MzNDMzA4LjEwMSAxNzQuOTAyIDMwOS4yNDQgMTc0LjU2IDMxMC4yMjEgMTczLjkxOEMzMTEuMTk5IDE3My4yNzYgMzExLjk2NyAxNzIuMzYyIDMxMi40MzMgMTcxLjI4OEMzMTIuODk5IDE3MC4yMTUgMzEzLjA0MSAxNjkuMDI4IDMxMi44NDMgMTY3Ljg3NUMzMTIuNjQ1IDE2Ni43MjEgMzEyLjExNSAxNjUuNjUxIDMxMS4zMTggMTY0Ljc5NUwyNTkuNjYgMTA5LjI5NFpNNS45OTcxOSA2OS40MjY3SDI1MC44ODRDMjUyLjUzIDY5LjQyNzUgMjU0LjE1OSA2OS4wODkgMjU1LjY2OSA2OC40MzJDMjU3LjE3OCA2Ny43NzUxIDI1OC41MzcgNjYuODEzOSAyNTkuNjYgNjUuNjA4MkwzMTEuMzE4IDEwLjEwNjlDMzEyLjExNSA5LjI1MTA3IDMxMi42NDUgOC4xODA1NiAzMTIuODQzIDcuMDI2OTVDMzEzLjA0MSA1Ljg3MzM0IDMxMi44OTkgNC42ODY4NiAzMTIuNDMzIDMuNjEzM0MzMTEuOTY3IDIuNTM5NzQgMzExLjE5OSAxLjYyNTg2IDMxMC4yMjEgMC45ODM5NDFDMzA5LjI0NCAwLjM0MjAyNiAzMDguMTAxIDMuOTUzMTRlLTA1IDMwNi45MzMgMEw2Mi4wNTk1IDBDNjAuNDE3OCAwLjAwMjc5ODY2IDU4Ljc5NDMgMC4zNDMxNCA1Ny4yODkzIDAuOTk5OTUzQzU1Ljc4NDMgMS42NTY3NyA1NC40Mjk5IDIuNjE2MDcgNTMuMzA5OSAzLjgxODQ3TDEuNjI1NzYgNTkuMzE5N0MwLjgyOTM2MSA2MC4xNzQ4IDAuMjk5MzU5IDYxLjI0NCAwLjEwMDc1MiA2Mi4zOTY0Qy0wLjA5Nzg1MzkgNjMuNTQ4OCAwLjA0MzU2OTggNjQuNzM0MiAwLjUwNzY3OSA2NS44MDczQzAuOTcxNzg5IDY2Ljg4MDMgMS43Mzg0MSA2Ny43OTQzIDIuNzEzNTIgNjguNDM3MkMzLjY4ODYzIDY5LjA4MDIgNC44Mjk4NCA2OS40MjQgNS45OTcxOSA2OS40MjY3WiIgZmlsbD0idXJsKCNwYWludDBfbGluZWFyXzQ3Nl8yNDMwKSIvPgo8L2c+CjxkZWZzPgo8bGluZWFyR3JhZGllbnQgaWQ9InBhaW50MF9saW5lYXJfNDc2XzI0MzAiIHgxPSIyNi40MTUiIHkxPSIyODcuMDU5IiB4Mj0iMjgzLjczNSIgeTI9Ii0yLjQ5NTc0IiBncmFkaWVudFVuaXRzPSJ1c2VyU3BhY2VPblVzZSI+CjxzdG9wIG9mZnNldD0iMC4wOCIgc3RvcC1jb2xvcj0iIzk5NDVGRiIvPgo8c3RvcCBvZmZzZXQ9IjAuMyIgc3RvcC1jb2xvcj0iIzg3NTJGMyIvPgo8c3RvcCBvZmZzZXQ9IjAuNSIgc3RvcC1jb2xvcj0iIzU0OTdENSIvPgo8c3RvcCBvZmZzZXQ9IjAuNiIgc3RvcC1jb2xvcj0iIzQzQjRDQSIvPgo8c3RvcCBvZmZzZXQ9IjAuNzIiIHN0b3AtY29sb3I9IiMyOEUwQjkiLz4KPHN0b3Agb2Zmc2V0PSIwLjk3IiBzdG9wLWNvbG9yPSIjMTlGQjlCIi8+CjwvbGluZWFyR3JhZGllbnQ+CjxjbGlwUGF0aCBpZD0iY2xpcDBfNDc2XzI0MzAiPgo8cmVjdCB3aWR0aD0iMzEyLjkzIiBoZWlnaHQ9IjI4MC4zNzciIGZpbGw9IndoaXRlIi8+CjwvY2xpcFBhdGg+CjwvZGVmcz4KPC9zdmc+Cg==&color=9945FF" /></a>
  <a href="https://www.npmjs.com/package/@nifty-oss/limestone"><img src="https://img.shields.io/npm/v/%40nifty-oss%2Flimestone?logo=npm&color=377CC0" /></a>
  <a href="https://crates.io/crates/limestone-client"><img src="https://img.shields.io/crates/v/limestone-client?logo=rust" /></a>
</p>

## Overview

Limestone enables the creation short-lived program derived addresses (PDAs) signers. These signers are used to create accounts which can be "safely" closed since the same account address (PDA signer) cannot be recreated after a time period, measured in terms of slots.

This feature is useful to avoid reusing an account for something completely different or in cases when applications or indexers store any information about the account, which could get out of sync if the account is closed and recreated on the same address.

You can use Limestone as a library or invoke its instruction &mdash; either directly from a client or through a cross program invocation &mdash; in your project. In both cases, you delegate the account creation to Limestone. The only difference is the program that signs for the PDA: when used as a library, your program is the signer; the Limestone program is the signer when its instruction is used.

> [!IMPORTANT]
> While PDA and PDA accounts are usually used interchangeably, a PDA is an address and not necessarily an account. More importantly, a PDA can be used to create an account owned by a different program than the one used to derive the PDA &mdash; one of the main uses of this is to allow programs to be signers.

### Using it as a library

From your project folder:

```bash
cargo add limestone
```

On your program, you replace the use of `system_instruction::create_account` with `limestone::create_account`:
```rust
use limestone::{Arguments, create_account};

create_account(
  program_id,
  Arguments {
    to: ctx.accounts.to,
    from: ctx.accounts.from,
    lamports,
    space,
    owner: Some(system_program::ID),
    slot,
  },
)?;
```
The arguments for the `create_account` are as follows:
* `program_id`:
   It is the address of your program (the account derivation will be done
   within the scope of the program).

* `from` (signer, writable):
  It is the funding account.

* `to` (writable):
  It is the account to be created (must be a PDA of `[from, slot]` derived from
  program_id).

* `lamports`:
  The lamports to be transferred to the new account (must be at least the amount
  needed for the account to be rent-exempt).

* `space`:
  The data size for the new account.

* `owner`:
  Optinal program that will own the new account (it default to `program_id` if
  omitted).

* `slot`:
  The slot number for the derivation (the slot needs to be within the valid range,
  i.e., not older than `current slot - TTL`).

> [!IMPORTANT]
> `create_account` uses the default `TTL` value of `150` slots. This is typically the number of slots that a `blockhash` is available and maximizes the chance of the account creation to succeed. You can use the `create_account_with_ttl` if you want to use a different `TTL` value – a lower `TTL` provides a shorter interval for the PDA signer to be available. At the same time, if your transaction is not executed within the `TTL` slots, it will fail.

### Using the `limestone` program

Limestone has a deployed program that can be used directly either from a client or another program and a companion client library with instruction builders. There are [JavaScript](clients/js/README.md) and [Rust](clients/rust/README.md) client packages.

#### JavaScript

Install the library using the package manager of your choice:
```bash
npm install @nifty-oss/limestone
```

The package contains an instruction builder:
```typescript
const slot = await client.rpc.getSlot().send();

const createAccountIx = await getCreateAccountInstructionAsync({
  from: payer,
  lamports: 500_000_000n,
  owner: address('AssetGtQBTSgm5s91d1RAQod5JmaZiJDxqsgtqrZud73'),
  space: 200,
  slot,
});
```

> [!NOTE]
> The package uses the new Solana [JavaScript SDK Technology Preview](https://www.npmjs.com/package/@solana/web3.js/v/2.0.0-preview.4). There is also a [package](clients/legacy/README.md) using the Umi framework.

#### Rust

From your project folder:

```bash
cargo add limestone-client
```

The `CreateAccountBuilder` builds the necessary instruction to create and account:
```rust
use limestone_client::{find_pda, instructions::CreateAccountBuilder};

let (pda, _) = find_pda(&payer.pubkey(), slot);

let create_ix = CreateAccountBuilder::new()
  .from(payer.pubkey())
  .to(pda)
  .lamports(5_000_000_000)
  .space(200)
  .owner(system_program::ID)
  .slot(slot)
  .instruction();
```

The same arguments used for the `create_account` function are used in the instruction builder.

When used in a program, the `CreateAccountCpiBuilder` can be used directly to invoke the `create_account` instruction:
```rust
use limestone_client::instructions::CreateAccountCpiBuilder;

CreateAccountCpiBuilder::new(program_info)
  .from(&payer_info)
  .to(&pda_info)
  .system_program(&system_program_info)
  .lamports(5_000_000_000)
  .space(200)
  .owner(system_program::ID)
  .slot(slot)
  .invoke()?;
```

> [!IMPORTANT]
> The `limestone` program uses a default of `150` slots as the `TTL` value.

## How it works

Limestone takes adavantage of how PDAs are handled in the runtime &mdash; a program can sign an instruction on behalf of PDAs derived from its program ID. This provides an important property: there is no private key generated for the address and, since the program is the only one that can sign on behalf of the PDA, there is an opportunity to control when it would do so. By limiting when this happens, we limit when a particular account can be created.

In Epemeris, PDAs are derived using a slot number and each slot value has a time-to-leave (`TTL`) associated with it. The `TTL` is used to validate whether the slot used to derive the PDA is too old or not. When the slot is deemed too old, Limestone will not sign the instruction &mdash; i.e., PDAs have a slot "range" (`slot > current slot - TTL`) defining when they can be used as signers.

This in practice restricts the ability to recreate the same account: if the account is closed after `TTL` slots have passed, there is no way to recreate the same account. This mitigates concerns of closeable mints, for example. A `mint` account can be created using an address generated by Limestone and be safely closeable without worries that it can be recreated in the future &mdash; there is no need to impose restrictions that a `mint` cannot be closed. This is particularly interesting for non-fungible protocols to prevent an NFT being recreated after it is burned &mdash; currently there are NFT standards that do not close the NFT account on burn to avoid account recreation, preventing users to fully recover rent funds and keeping unnecessary account state.

The advantage of using a slot value is that there is no write contention since Limestone does not require a "global" account. An alternative way to achieved something similar is to use a program with a sequential `counter` stored on an account, where the `counter` is incremented every time a new account is created to provide an unique value. The drawback of this approach is that there is a write contention on the account storing the `counter`, which makes it more difficult for different clients to use it concurrently.

## Limitations

Although the use `TTL` defines a time period where the account creation is allowed &mdash; `150` slots is approximately 1 minute 19 seconds assuming `400`ms block times &mdash; it does not guarantee that the account is not closed and recreated between that interval. Additionally, it does not prevent an account being created, closed and recreated on the same transaction.

For protocols that need such guarantee, an addional restriction should be added when closing an account that should not be recreated. The protocol should store the `slot` value used on the account derivation and validate that the account is being closed after `slot + TTL` &mdash; this will prevent the account recreation since the `slot` value will be too old to generate a PDA signer.

Since a slot value is part of the derivation of the account, it cannot be easily used in scenarios where durable nonces are required to build transactions. It is very likely that the slot value will be invalid when the transaction is signed at a point in the future. This limitation is not due to the approach of using a PDA signer &mdash; it arises from the fact that the slot expires in the same way that a blockhash expires. The alternative in this case is to use an approach where the slot in the derivation is replaced by the nonce value, which will provide a similar guarantee that a derivation is only valid for a particular nonce value.

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
