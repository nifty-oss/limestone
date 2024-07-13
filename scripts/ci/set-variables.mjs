#!/usr/bin/env zx
import { getRustfmtToolchain, getClippyToolchain } from '../utils.mjs';

const rustfmtToolchain = getRustfmtToolchain();
const clippyToolchain = getClippyToolchain();

await $`echo "RUSTFMT_TOOLCHAIN=${rustfmtToolchain}" >> $GITHUB_ENV`;
await $`echo "CLIPPY_TOOLCHAIN=${clippyToolchain}" >> $GITHUB_ENV`;
