import { Context, Pda, PublicKey } from '@metaplex-foundation/umi';
import {
  publicKey as publicKeySerializer,
  u64,
} from '@metaplex-foundation/umi/serializers';

export function findAccountPda(
  context: Pick<Context, 'eddsa' | 'programs'>,
  seeds: {
    /** Funding account */
    from: PublicKey;
    /** Slot value */
    slot: number | bigint;
    /** Base public key for the account derivation */
    base?: PublicKey;
  }
): Pda {
  const programId = context.programs.getPublicKey(
    'limestone',
    'LMSToZQenurAeAutm239hcJBCgsaPNaJhNC7nJhrtdB'
  );
  if (seeds.base) {
    return context.eddsa.findPda(programId, [
      publicKeySerializer().serialize(seeds.from),
      u64().serialize(Number(seeds.slot)),
      publicKeySerializer().serialize(seeds.base),
    ]);
  }
  return context.eddsa.findPda(programId, [
    publicKeySerializer().serialize(seeds.from),
    u64().serialize(Number(seeds.slot)),
  ]);
}
