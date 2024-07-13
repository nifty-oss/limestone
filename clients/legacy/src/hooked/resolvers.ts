import { Context, PublicKey } from '@metaplex-foundation/umi';
import { ResolvedAccounts, expectPublicKey } from '../generated';
import { findAccountPda } from '../pda';

export const resolveAccount = (
  context: Pick<Context, 'eddsa' | 'programs'>,
  accounts: ResolvedAccounts,
  args: { slot: number | bigint },
  _programId: PublicKey,
  isWritable: boolean
) => {
  return {
    value: findAccountPda(context, {
      base: expectPublicKey(accounts.base.value),
      slot: args.slot,
    })[0],
    isWritable,
  };
};
