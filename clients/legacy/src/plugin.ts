import { UmiPlugin } from '@metaplex-foundation/umi';
import { createEphemerisProgram } from './generated';

export const ephemeris = (): UmiPlugin => ({
  install(umi) {
    umi.programs.add(createEphemerisProgram(), false);
  },
});
