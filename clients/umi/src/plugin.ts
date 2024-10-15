import { UmiPlugin } from '@metaplex-foundation/umi';
import { createLimestoneProgram } from './generated';

export const limestone = (): UmiPlugin => ({
  install(umi) {
    umi.programs.add(createLimestoneProgram(), false);
  },
});
