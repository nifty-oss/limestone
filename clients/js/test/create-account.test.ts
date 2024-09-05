import {
  address,
  appendTransactionMessageInstruction,
  fetchEncodedAccount,
  pipe,
} from '@solana/web3.js';
import test from 'ava';
import { findAccountPda, getCreateAccountInstructionAsync } from '../src';
import {
  createDefaultSolanaClient,
  createDefaultTransaction,
  generateAddress,
  generateKeyPairSignerWithSol,
  signAndSendTransaction,
} from './_setup';

test('it creates a new account', async (t) => {
  // Given an payer key pair with some SOL and the current slot number.
  const client = createDefaultSolanaClient();
  const payer = await generateKeyPairSignerWithSol(client);
  const slot = await client.rpc.getSlot().send();

  // When we create a new account.
  const createAccountIx = await getCreateAccountInstructionAsync({
    from: payer,
    lamports: 500_000_000n,
    owner: address('11111111111111111111111111111111'),
    space: 200,
    slot,
  });

  await pipe(
    await createDefaultTransaction(client, payer),
    (tx) => appendTransactionMessageInstruction(createAccountIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  // Then we expect the account to exist.

  const [pda] = await findAccountPda({ from: payer.address, slot, base: null });
  const account = await fetchEncodedAccount(client.rpc, pda);

  t.like(account, {
    address: pda,
    lamports: 500_000_000n,
    programAddress: address('11111111111111111111111111111111'),
  });
});

test('it creates a new account with seeded address derivation', async (t) => {
  // Given an payer key pair with some SOL and the current slot number.
  const client = createDefaultSolanaClient();
  const payer = await generateKeyPairSignerWithSol(client);
  const slot = await client.rpc.getSlot().send();

  // And a base public key for the derivation.
  const base = await generateAddress();

  // When we create a new account.
  const createAccountIx = await getCreateAccountInstructionAsync({
    from: payer,
    base,
    lamports: 500_000_000n,
    owner: address('AssetGtQBTSgm5s91d1RAQod5JmaZiJDxqsgtqrZud73'),
    space: 200,
    slot,
  });

  await pipe(
    await createDefaultTransaction(client, payer),
    (tx) => appendTransactionMessageInstruction(createAccountIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  // Then we expect the account to exist.

  const [pda] = await findAccountPda({ from: payer.address, base, slot });
  const account = await fetchEncodedAccount(client.rpc, pda);

  t.like(account, {
    address: pda,
    lamports: 500_000_000n,
    programAddress: address('AssetGtQBTSgm5s91d1RAQod5JmaZiJDxqsgtqrZud73'),
  });
});
