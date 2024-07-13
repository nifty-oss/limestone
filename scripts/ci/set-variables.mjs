#!/usr/bin/env zx
import { getRustfmtToolchain, getClippyToolchain } from '../utils.mjs';

const rustfmtToolchain = getRustfmtToolchain();
const clippyToolchain = getClippyToolchain();

if (rustfmtToolchain) {
  await $`echo "RUSTFMT_TOOLCHAIN=${rustfmtToolchain}" >> $GITHUB_ENV`;
}

if (clippyToolchain) {
  await $`echo "CLIPPY_TOOLCHAIN=${clippyToolchain}" >> $GITHUB_ENV`;
}
