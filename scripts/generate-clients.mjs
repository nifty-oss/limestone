#!/usr/bin/env zx
import "zx/globals";
import * as k from "kinobi";
import { rootNodeFromAnchor } from "@kinobi-so/nodes-from-anchor";
import { renderVisitor as renderJavaScriptVisitor } from "@kinobi-so/renderers-js";
import { renderVisitor as renderLegacyJavaScriptVisitor } from "@kinobi-so/renderers-js-umi";
import { renderVisitor as renderRustVisitor } from "@kinobi-so/renderers-rust";
import { getAllProgramIdls } from "./utils.mjs";

// Instanciate Kinobi.
const [idl, ...additionalIdls] = getAllProgramIdls().map((idl) =>
  rootNodeFromAnchor(require(idl))
);
const kinobi = k.createFromRoot(idl, additionalIdls);

// Update programs.
kinobi.update(
  k.updateProgramsVisitor({
    limestoneProgram: { name: "limestone" },
  })
);

// Add pda information.
kinobi.update(
  k.bottomUpTransformerVisitor([
    {
      select: "[programNode]limestone",
      transform: (node) => {
        k.assertIsNode(node, "programNode");
        return {
          ...node,
          pdas: [
            k.pdaNode({
              name: "account",
              seeds: [
                k.variablePdaSeedNode(
                  "base",
                  k.publicKeyTypeNode(),
                  "Base public key for the address derivation"
                ),
                k.variablePdaSeedNode(
                  "slot",
                  k.numberTypeNode("u64"),
                  "Slot for the address derivation"
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
kinobi.update(
  k.updateInstructionsVisitor({
    createAccount: {
      accounts: {
        base: {
          defaultValue: k.accountValueNode("from"),
        },
        to: {
          defaultValue: k.resolverValueNode("resolveAccount", {
            dependsOn: [
              k.accountValueNode("base"),
              k.argumentValueNode("slot"),
            ],
          }),
        },
      },
    },
  })
);

// Render JavaScript.
const jsClient = path.join(__dirname, "..", "clients", "js");
kinobi.accept(
  renderJavaScriptVisitor(path.join(jsClient, "src", "generated"), {
    prettier: require(path.join(jsClient, ".prettierrc.json")),
    asyncResolvers: ["resolveAccount"],
  })
);

// Render legacy JavaScript.
const jsLegacyClient = path.join(__dirname, "..", "clients", "legacy");
kinobi.accept(
  renderLegacyJavaScriptVisitor(path.join(jsLegacyClient, "src", "generated"), {
    prettier: require(path.join(jsLegacyClient, ".prettierrc.json")),
  })
);

// Render Rust.
const rustClient = path.join(__dirname, "..", "clients", "rust");
kinobi.accept(
  renderRustVisitor(path.join(rustClient, "src", "generated"), {
    formatCode: true,
    crateFolder: rustClient,
  })
);
