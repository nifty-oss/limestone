import { generateSigner, publicKey } from '@metaplex-foundation/umi';
import { generateSignerWithSol } from '@metaplex-foundation/umi-bundle-tests';
import test from 'ava';
import { createAccount, findAccountPda } from '../src';
import { createUmi } from './_setup';

test('it creates a new account', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const payer = await generateSignerWithSol(umi);
  const slot = await umi.rpc.getSlot();

  // When we create a new account.
  await createAccount(umi, {
    from: payer,
    lamports: 500_000_000n,
    owner: publicKey('AssetGtQBTSgm5s91d1RAQod5JmaZiJDxqsgtqrZud73'),
    space: 200,
    slot,
  }).sendAndConfirm(umi);

  // Then we expect the account to exist.
  const pda = publicKey(findAccountPda(umi, { base: payer.publicKey, slot }));
  const account = await umi.rpc.getAccount(pda);

  t.like(account, {
    publicKey: pda,
    lamports: {
      basisPoints: 500_000_000n,
    },
    owner: publicKey('AssetGtQBTSgm5s91d1RAQod5JmaZiJDxqsgtqrZud73'),
  });
});

test('it creates a new account with a base address derivation', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const payer = await generateSignerWithSol(umi);
  const slot = await umi.rpc.getSlot();

  // And a base signer.
  const base = await generateSigner(umi);

  // When we create a new account.
  await createAccount(umi, {
    from: payer,
    base,
    lamports: 500_000_000n,
    owner: publicKey('AssetGtQBTSgm5s91d1RAQod5JmaZiJDxqsgtqrZud73'),
    space: 200,
    slot,
  }).sendAndConfirm(umi);

  // Then we expect the account to exist.
  const pda = publicKey(findAccountPda(umi, { base: base.publicKey, slot }));
  const account = await umi.rpc.getAccount(pda);

  t.like(account, {
    publicKey: pda,
    lamports: {
      basisPoints: 500_000_000n,
    },
    owner: publicKey('AssetGtQBTSgm5s91d1RAQod5JmaZiJDxqsgtqrZud73'),
  });
});
