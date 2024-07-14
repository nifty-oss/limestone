<h1 align="center">
  <code>ephemeris</code>
</h1>
<p align="center">
  <img width="400" alt="Ephemeris" src="https://github.com/user-attachments/assets/a889bf96-5e3f-40de-b343-b8135dfe19a9" />
</p>
<p align="center">
  Generate slot-based unique program derived addresses.
</p>

<p align="center">
  <a href="https://explorer.solana.com/address/EPHSqv4H9HG5xy1kQaQaLN14zyBP36Jzq7hrQ2ZEZbBj"><img src="https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fraw.githubusercontent.com%2Fnifty-oss%2Fephemeris%2Fmain%2Fprogram%2Fidl.json&query=%24.version&label=program&logo=data:image/svg%2bxml;base64,PHN2ZyB3aWR0aD0iMzEzIiBoZWlnaHQ9IjI4MSIgdmlld0JveD0iMCAwIDMxMyAyODEiIGZpbGw9Im5vbmUiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+CjxnIGNsaXAtcGF0aD0idXJsKCNjbGlwMF80NzZfMjQzMCkiPgo8cGF0aCBkPSJNMzExLjMxOCAyMjEuMDU3TDI1OS42NiAyNzYuNTU4QzI1OC41MzcgMjc3Ljc2NCAyNTcuMTc4IDI3OC43MjUgMjU1LjY2OSAyNzkuMzgyQzI1NC4xNTkgMjgwLjAzOSAyNTIuNTMgMjgwLjM3OCAyNTAuODg0IDI4MC4zNzdINS45OTcxOUM0LjgyODcgMjgwLjM3NyAzLjY4NTY4IDI4MC4wMzUgMi43MDg1NSAyNzkuMzkzQzEuNzMxNDMgMjc4Ljc1MSAwLjk2Mjc3MSAyNzcuODM3IDAuNDk3MDIgMjc2Ljc2NEMwLjAzMTI2OTEgMjc1LjY5IC0wLjExMTI4NiAyNzQuNTA0IDAuMDg2ODcxMiAyNzMuMzVDMC4yODUwMjggMjcyLjE5NiAwLjgxNTI2NSAyNzEuMTI2IDEuNjEyNDMgMjcwLjI3TDUzLjMwOTkgMjE0Ljc2OUM1NC40Mjk5IDIxMy41NjYgNTUuNzg0MyAyMTIuNjA3IDU3LjI4OTMgMjExLjk1QzU4Ljc5NDMgMjExLjI5MyA2MC40MTc4IDIxMC45NTMgNjIuMDU5NSAyMTAuOTVIMzA2LjkzM0MzMDguMTAxIDIxMC45NSAzMDkuMjQ0IDIxMS4yOTIgMzEwLjIyMSAyMTEuOTM0QzMxMS4xOTkgMjEyLjU3NiAzMTEuOTY3IDIxMy40OSAzMTIuNDMzIDIxNC41NjRDMzEyLjg5OSAyMTUuNjM3IDMxMy4wNDEgMjE2LjgyNCAzMTIuODQzIDIxNy45NzdDMzEyLjY0NSAyMTkuMTMxIDMxMi4xMTUgMjIwLjIwMSAzMTEuMzE4IDIyMS4wNTdaTTI1OS42NiAxMDkuMjk0QzI1OC41MzcgMTA4LjA4OCAyNTcuMTc4IDEwNy4xMjcgMjU1LjY2OSAxMDYuNDdDMjU0LjE1OSAxMDUuODEzIDI1Mi41MyAxMDUuNDc0IDI1MC44ODQgMTA1LjQ3NUg1Ljk5NzE5QzQuODI4NyAxMDUuNDc1IDMuNjg1NjggMTA1LjgxNyAyLjcwODU1IDEwNi40NTlDMS43MzE0MyAxMDcuMTAxIDAuOTYyNzcxIDEwOC4wMTUgMC40OTcwMiAxMDkuMDg4QzAuMDMxMjY5MSAxMTAuMTYyIC0wLjExMTI4NiAxMTEuMzQ4IDAuMDg2ODcxMiAxMTIuNTAyQzAuMjg1MDI4IDExMy42NTYgMC44MTUyNjUgMTE0LjcyNiAxLjYxMjQzIDExNS41ODJMNTMuMzA5OSAxNzEuMDgzQzU0LjQyOTkgMTcyLjI4NiA1NS43ODQzIDE3My4yNDUgNTcuMjg5MyAxNzMuOTAyQzU4Ljc5NDMgMTc0LjU1OSA2MC40MTc4IDE3NC44OTkgNjIuMDU5NSAxNzQuOTAySDMwNi45MzNDMzA4LjEwMSAxNzQuOTAyIDMwOS4yNDQgMTc0LjU2IDMxMC4yMjEgMTczLjkxOEMzMTEuMTk5IDE3My4yNzYgMzExLjk2NyAxNzIuMzYyIDMxMi40MzMgMTcxLjI4OEMzMTIuODk5IDE3MC4yMTUgMzEzLjA0MSAxNjkuMDI4IDMxMi44NDMgMTY3Ljg3NUMzMTIuNjQ1IDE2Ni43MjEgMzEyLjExNSAxNjUuNjUxIDMxMS4zMTggMTY0Ljc5NUwyNTkuNjYgMTA5LjI5NFpNNS45OTcxOSA2OS40MjY3SDI1MC44ODRDMjUyLjUzIDY5LjQyNzUgMjU0LjE1OSA2OS4wODkgMjU1LjY2OSA2OC40MzJDMjU3LjE3OCA2Ny43NzUxIDI1OC41MzcgNjYuODEzOSAyNTkuNjYgNjUuNjA4MkwzMTEuMzE4IDEwLjEwNjlDMzEyLjExNSA5LjI1MTA3IDMxMi42NDUgOC4xODA1NiAzMTIuODQzIDcuMDI2OTVDMzEzLjA0MSA1Ljg3MzM0IDMxMi44OTkgNC42ODY4NiAzMTIuNDMzIDMuNjEzM0MzMTEuOTY3IDIuNTM5NzQgMzExLjE5OSAxLjYyNTg2IDMxMC4yMjEgMC45ODM5NDFDMzA5LjI0NCAwLjM0MjAyNiAzMDguMTAxIDMuOTUzMTRlLTA1IDMwNi45MzMgMEw2Mi4wNTk1IDBDNjAuNDE3OCAwLjAwMjc5ODY2IDU4Ljc5NDMgMC4zNDMxNCA1Ny4yODkzIDAuOTk5OTUzQzU1Ljc4NDMgMS42NTY3NyA1NC40Mjk5IDIuNjE2MDcgNTMuMzA5OSAzLjgxODQ3TDEuNjI1NzYgNTkuMzE5N0MwLjgyOTM2MSA2MC4xNzQ4IDAuMjk5MzU5IDYxLjI0NCAwLjEwMDc1MiA2Mi4zOTY0Qy0wLjA5Nzg1MzkgNjMuNTQ4OCAwLjA0MzU2OTggNjQuNzM0MiAwLjUwNzY3OSA2NS44MDczQzAuOTcxNzg5IDY2Ljg4MDMgMS43Mzg0MSA2Ny43OTQzIDIuNzEzNTIgNjguNDM3MkMzLjY4ODYzIDY5LjA4MDIgNC44Mjk4NCA2OS40MjQgNS45OTcxOSA2OS40MjY3WiIgZmlsbD0idXJsKCNwYWludDBfbGluZWFyXzQ3Nl8yNDMwKSIvPgo8L2c+CjxkZWZzPgo8bGluZWFyR3JhZGllbnQgaWQ9InBhaW50MF9saW5lYXJfNDc2XzI0MzAiIHgxPSIyNi40MTUiIHkxPSIyODcuMDU5IiB4Mj0iMjgzLjczNSIgeTI9Ii0yLjQ5NTc0IiBncmFkaWVudFVuaXRzPSJ1c2VyU3BhY2VPblVzZSI+CjxzdG9wIG9mZnNldD0iMC4wOCIgc3RvcC1jb2xvcj0iIzk5NDVGRiIvPgo8c3RvcCBvZmZzZXQ9IjAuMyIgc3RvcC1jb2xvcj0iIzg3NTJGMyIvPgo8c3RvcCBvZmZzZXQ9IjAuNSIgc3RvcC1jb2xvcj0iIzU0OTdENSIvPgo8c3RvcCBvZmZzZXQ9IjAuNiIgc3RvcC1jb2xvcj0iIzQzQjRDQSIvPgo8c3RvcCBvZmZzZXQ9IjAuNzIiIHN0b3AtY29sb3I9IiMyOEUwQjkiLz4KPHN0b3Agb2Zmc2V0PSIwLjk3IiBzdG9wLWNvbG9yPSIjMTlGQjlCIi8+CjwvbGluZWFyR3JhZGllbnQ+CjxjbGlwUGF0aCBpZD0iY2xpcDBfNDc2XzI0MzAiPgo8cmVjdCB3aWR0aD0iMzEyLjkzIiBoZWlnaHQ9IjI4MC4zNzciIGZpbGw9IndoaXRlIi8+CjwvY2xpcFBhdGg+CjwvZGVmcz4KPC9zdmc+Cg==&color=9945FF" /></a>
  <a href="https://www.npmjs.com/package/@nifty-oss/ephemeris"><img src="https://img.shields.io/npm/v/%40nifty-oss%2Fephemeris?logo=npm&color=377CC0" /></a>
  <a href="https://crates.io/crates/ephemeris-client"><img src="https://img.shields.io/crates/v/ephemeris-client?logo=rust" /></a>
</p>

## Overview

Ephemeris enables the creation short-lived program derived addresses (PDAs) signers. These signers are used to create accounts which can be "safely" closed since the same account address (PDA signer) cannot be recreated after a time period.

You can use Ephemeris as a library or invoke its instruction &mdash; either directly from a client or through a cross program invocation &mdash; in your project. In both cases, you delegate the account creation to Ephemeris.

### Using it as a library

From your project folder:

```bash
cargo add ephemeris
```

On your program, you replace the use of `system_instruction::create_account` with `ephemeris::create_account`:
```rust
ephemeris::create_account(
  program_id,
  Arguments {
    to: ctx.accounts.to,
    from: ctx.accounts.from,
    base: ctx.accounts.base.map(|account| *account.key),
    slot,
    lamports,
    space,
    owner,
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

> [!IMPORTANT]
> `create_account` uses the default `TTL` value of `150` slots. This is typically the number of slots that a `blockhash` is available and maximizes the chance of the account creation to succeed. You can use the `create_account_with_ttl` if you want to use a different `TTL` value – a lower `TTL` provides a shorter interval for the PDA signer to be available. At the same time, if your transaction is not executed within the `TTL` slots, it will fail.

### Invoking `ephemeris` program

Ephemeris has a deployed program that can be used directly either from a client or another program and a companion client library with instruction builders.

From your project folder:

```bash
cargo add ephemeris-client
```

The `CreateAccountBuilder` builds the necessary instruction to create and account:
```rust
use ephemeris_client::{find_pda, instructions::CreateAccountBuilder};

let (pda, _) = find_pda(&payer.pubkey(), slot);

let create_ix = CreateAccountBuilder::new()
  .from(payer.pubkey())
  .to(pda)
  .slot(slot)
  .lamports(5_000_000_000)
  .space(200)
  .owner(system_program::ID)
  .instruction();
```

The same arguments used for the `create_account` function are used in the instruction builder.

When used in a program, the `CreateAccountCpiBuilder` can be used directly to invoke the `create_account` instruction:
```rust
use ephemeris_client::instructions::CreateAccountCpiBuilder;

CreateAccountCpiBuilder::new(program_info)
  .from(&payer_info)
  .to(&pda_info)
  .system_program(&system_program_info)
  .slot(slot)
  .lamports(5_000_000_000)
  .space(200)
  .owner(system_program::ID)
  .invoke()?;
```

> [!IMPORTANT]
> The `ephemeris` program uses a default of `150` slots as the `TTL` value.

## How it works

Ephemeris takes adavantage of how PDAs are handled in the runtime &mdash; a program can "sign" on behalf of the PDAs derived from its program ID. There is no private key generated for the address and a PDA can only be used as a signer by the program that derives it. This guarantees that only the program can sign for a `create_account` instruction. By limiting when the program signs for the account creation, we can limit when an account can be created. A natural way to count time in a blockchain context is using the bock slot number.

> [!NOTE]
> While PDA and PDA accounts are usually used interchangeably, a PDA is an address and not necessarily an account. More importantly, a PDA can be used to create an account owned by a different program than the only used to derive the PDA.

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
