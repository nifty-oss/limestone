#!/usr/bin/env zx
import 'zx/globals';
import {
  getClippyToolchain,
  getProgramFolders,
  getToolchainArg,
  workingDirectory,
} from '../utils.mjs';

const toolchain = getToolchainArg(getClippyToolchain());
// Lint the programs using clippy.
for (const folder of getProgramFolders()) {
  cd(`${path.join(workingDirectory, folder)}`);
  await $`cargo ${toolchain} clippy ${process.argv.slice(3)}`;
}
