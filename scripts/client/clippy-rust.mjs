#!/usr/bin/env zx
import 'zx/globals';
import {
  getClippyToolchain,
  getToolchainArg,
  workingDirectory,
} from '../utils.mjs';

const toolchain = getToolchainArg(getClippyToolchain());
// Check the client using clippy.
cd(path.join(workingDirectory, 'clients', 'rust'));
await $`cargo ${toolchain} clippy ${process.argv.slice(3)}`;
