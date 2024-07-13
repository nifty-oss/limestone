#!/usr/bin/env zx
import 'zx/globals';
import {
  getClippyToolchain,
  getToolchainArg,
  workingDirectory,
} from '../utils.mjs';

const toolchain = getToolchainArg(getClippyToolchain());
// Format the client using rust fmt.
cd(path.join(workingDirectory, 'clients', 'rust'));
await $`cargo ${toolchain} fmt ${process.argv.slice(3)}`;
