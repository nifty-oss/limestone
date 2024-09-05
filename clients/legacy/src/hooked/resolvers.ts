import { Context, PublicKey } from '@metaplex-foundation/umi';
import { ResolvedAccounts, expectPublicKey } from '../generated';
import { findAccountPda } from '../pda';

export const resolveAccount = (
  context: Pick<Context, 'eddsa' | 'programs'>,
  accounts: ResolvedAccounts,
  args: { slot: number | bigint },
  _programId: PublicKey,
  isWritable: boolean
) => ({
  value: findAccountPda(context, {
    from: expectPublicKey(accounts.from.value),
    slot: args.slot,
    base: accounts.base?.value
      ? expectPublicKey(accounts.base.value)
      : undefined,
  })[0],
  isWritable,
});
