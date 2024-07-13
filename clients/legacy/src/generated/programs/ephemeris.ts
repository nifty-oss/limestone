/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import {
  ClusterFilter,
  Context,
  Program,
  PublicKey,
} from '@metaplex-foundation/umi';
import {
  getEphemerisErrorFromCode,
  getEphemerisErrorFromName,
} from '../errors';

export const EPHEMERIS_PROGRAM_ID =
  'EPHSqv4H9HG5xy1kQaQaLN14zyBP36Jzq7hrQ2ZEZbBj' as PublicKey<'EPHSqv4H9HG5xy1kQaQaLN14zyBP36Jzq7hrQ2ZEZbBj'>;

export function createEphemerisProgram(): Program {
  return {
    name: 'ephemeris',
    publicKey: EPHEMERIS_PROGRAM_ID,
    getErrorFromCode(code: number, cause?: Error) {
      return getEphemerisErrorFromCode(code, this, cause);
    },
    getErrorFromName(name: string, cause?: Error) {
      return getEphemerisErrorFromName(name, this, cause);
    },
    isOnCluster() {
      return true;
    },
  };
}

export function getEphemerisProgram<T extends Program = Program>(
  context: Pick<Context, 'programs'>,
  clusterFilter?: ClusterFilter
): T {
  return context.programs.get<T>('ephemeris', clusterFilter);
}

export function getEphemerisProgramId(
  context: Pick<Context, 'programs'>,
  clusterFilter?: ClusterFilter
): PublicKey {
  return context.programs.getPublicKey(
    'ephemeris',
    EPHEMERIS_PROGRAM_ID,
    clusterFilter
  );
}
