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
  let base = null;
  if (accounts.base?.value) {
    base = expectAddress(accounts.base?.value);
  }
  return {
    value: await findAccountPda({
      from: expectAddress(accounts.from.value),
      base,
      slot: args.slot,
    }),
  };
};
