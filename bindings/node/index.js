import {
  conformanceDeclare,
  initSync,
  parseDocument,
  specVersion,
  validateContract,
} from "@eddiethedean/dtcs-wasm";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const require = createRequire(import.meta.url);
const wasmPath = require.resolve("@eddiethedean/dtcs-wasm/pkg/dtcs_wasm_bg.wasm");
const wasmBytes = readFileSync(wasmPath);
initSync({ module: wasmBytes });

export { conformanceDeclare, parseDocument, specVersion, validateContract };

export function parse(content, format = "yaml") {
  const bytes =
    typeof content === "string" ? new TextEncoder().encode(content) : content;
  return parseDocument(bytes, format);
}

export function validate(contract) {
  return validateContract(contract);
}

export function conformanceDeclareAll() {
  return conformanceDeclare();
}

export function conformanceRun(profile = "integrated-platform") {
  throw new Error(
    "conformanceRun is not available in Node/WASM bindings; use the dtcs CLI or Python package",
  );
}

export const SPEC_VERSION = specVersion();
