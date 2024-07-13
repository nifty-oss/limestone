import { ProgramDerivedAddress } from '@solana/web3.js';
import { ResolvedAccount, expectAddress } from '../generated/shared';
import { findAccountPda } from '../generated';

export const resolveAccount = async ({
  accounts,
  args,
}: {
  accounts: Record<string, ResolvedAccount>;
  args: { slot: number | bigint };
}): Promise<Partial<{ value: ProgramDerivedAddress | null }>> => {
  return {
    value: await findAccountPda({
      base: expectAddress(accounts.base?.value),
      slot: args.slot,
    }),
  };
};
