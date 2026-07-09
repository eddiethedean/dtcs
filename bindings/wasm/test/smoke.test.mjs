import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
import test from "node:test";

const root = dirname(fileURLToPath(import.meta.url));
const pkgRoot = join(root, "..", "pkg");

test("wasm bindings smoke test", async (t) => {
  let wasm;
  try {
    wasm = await import(join(pkgRoot, "dtcs_wasm.js"));
  } catch (err) {
    t.skip(`wasm package not built: ${err}`);
    return;
  }

  await wasm.default();
  assert.equal(wasm.specVersion(), "1.0.0-draft");

  const fixture = readFileSync(
    join(root, "..", "..", "tests", "fixtures", "valid_customer.yaml"),
  );
  const parsed = wasm.parseDocument(fixture, "yaml");
  assert.ok(parsed.contract);
  assert.ok(parsed.report);

  const declaration = wasm.conformanceDeclare();
  assert.equal(declaration.primaryProfile, "integrated-platform");
});
