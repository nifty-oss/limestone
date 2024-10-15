#!/usr/bin/env zx
import "zx/globals";
import * as c from "codama";
import { rootNodeFromAnchor } from "@codama/nodes-from-anchor";
import { renderVisitor as renderJavaScriptVisitor } from "@codama/renderers-js";
import { renderVisitor as renderUmiJavaScriptVisitor } from "@codama/renderers-js-umi";
import { renderVisitor as renderRustVisitor } from "@codama/renderers-rust";
import { getAllProgramIdls } from "./utils.mjs";

// Instanciate Codama.
const [idl, ...additionalIdls] = getAllProgramIdls().map((idl) =>
  rootNodeFromAnchor(require(idl))
);
const codama = c.createFromRoot(idl, additionalIdls);

// Update programs.
codama.update(
  c.updateProgramsVisitor({
    limestoneProgram: { name: "limestone" },
  })
);

// Add pda information.
codama.update(
  c.bottomUpTransformerVisitor([
    {
      select: "[programNode]limestone",
      transform: (node) => {
        c.assertIsNode(node, "programNode");
        return {
          ...node,
          pdas: [
            c.pdaNode({
              name: "account",
              seeds: [
                c.variablePdaSeedNode(
                  "from",
                  c.publicKeyTypeNode(),
                  "Funding account"
                ),
                c.variablePdaSeedNode(
                  "slot",
                  c.numberTypeNode("u64"),
                  "Slot for the address derivation"
                ),
                c.variablePdaSeedNode(
                  "base",
                  c.remainderOptionTypeNode(c.publicKeyTypeNode()),
                  "Base public key for the account derivation"
                ),
              ],
            }),
          ],
        };
      },
    },
  ])
);

// Update instructions.
codama.update(
  c.updateInstructionsVisitor({
    createAccount: {
      accounts: {
        to: {
          defaultValue: c.resolverValueNode("resolveAccount", {
            dependsOn: [
              c.accountValueNode("from"),
              c.accountValueNode("base"),
              c.argumentValueNode("slot"),
            ],
          }),
        },
      },
    },
  })
);

// Render JavaScript.
const jsClient = path.join(__dirname, "..", "clients", "js");
codama.accept(
  renderJavaScriptVisitor(path.join(jsClient, "src", "generated"), {
    prettier: require(path.join(jsClient, ".prettierrc.json")),
    asyncResolvers: ["resolveAccount"],
  })
);

// Render Umi JavaScript.
const jsUmiClient = path.join(__dirname, "..", "clients", "umi");
codama.accept(
  renderUmiJavaScriptVisitor(path.join(jsUmiClient, "src", "generated"), {
    prettier: require(path.join(jsUmiClient, ".prettierrc.json")),
  })
);

// Render Rust.
const rustClient = path.join(__dirname, "..", "clients", "rust");
codama.accept(
  renderRustVisitor(path.join(rustClient, "src", "generated"), {
    formatCode: true,
    crateFolder: rustClient,
  })
);
