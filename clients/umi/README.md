# <img height="50" alt="limestone" src="https://github.com/user-attachments/assets/53b09445-dba6-43c4-9bdf-f4df9ab677a3"/>

A Umi-based JavaScript client library for Limestone [program](https://github.com/nifty-oss/limestone).

Limestone enables the creation short-lived program derived address (PDA) signers. These signers are used to create accounts which can be "safely" closed since the same account address signer cannot be recreated after a time period &mdash; `time` in this case measured in terms of slots.

This feature is useful to avoid reusing an account for something completely different, which can create problems for applications and off-chain services that store any information about the account. Therefore, accounts can be closed, avoiding bloating account state and returning all rent funds &mdash; this enables the use of accounts to represent "ephemeral" concepts (e.g., receipts, tickets) without incurring unnecessary costs.

## Getting Started

1. First, if you're not already using Umi, [follow these instructions to install the Metaplex Umi framework](https://github.com/metaplex-foundation/umi/blob/main/docs/installation.md).

2. Next, install this library using the package manager of your choice.
   ```sh
   npm install @nifty-oss/limestone-umi
   ```
2. Finally, register the library with your Umi instance.
   ```ts
   import { limestone } from '@nifty-oss/limestone-umi';
   umi.use(limestone());
   ```

The library contains a builder for the `CreateAccount` instruction of `limestone` program:
```typescript
import { createAccount } from '@nifty-oss/limestone-umi';

const slot = await umi.rpc.getSlot();

await createAccount(umi, {
  from: payer,
  slot,
  lamports: 500_000_000n,
  space: 200,
  owner: publicKey('AssetGtQBTSgm5s91d1RAQod5JmaZiJDxqsgtqrZud73'),
}).sendAndConfirm(umi);
```

The arguments required to create an account are as follows:

* `from` (signer):
  It is the funding account.

* `to`:
  It is the account to be created (must be a PDA of `[base, slot]` derived from
  program_id). The instruction builder sets the value for this account when it is
  not directly provided.

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

> This library uses the [Metaplex Umi framework](https://github.com/metaplex-foundation/umi). There is also a version of the library using the new Solana [JavaScript SDK](https://www.npmjs.com/package/@solana/web3.js/v/2.0.0-rc.1).

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
