import { Context, Pda, PublicKey } from '@metaplex-foundation/umi';
import {
  publicKey as publicKeySerializer,
  u64,
} from '@metaplex-foundation/umi/serializers';

export function findAccountPda(
  context: Pick<Context, 'eddsa' | 'programs'>,
  seeds: {
    /** The base address of the derivation */
    base: PublicKey;
    /** Slot value */
    slot: number | bigint;
  }
): Pda {
  const programId = context.programs.getPublicKey(
    'limestone',
    'EPHSqv4H9HG5xy1kQaQaLN14zyBP36Jzq7hrQ2ZEZbBj'
  );
  return context.eddsa.findPda(programId, [
    publicKeySerializer().serialize(seeds.base),
    u64().serialize(Number(seeds.slot)),
  ]);
}
