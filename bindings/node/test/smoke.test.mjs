import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
import test from "node:test";

const root = dirname(fileURLToPath(import.meta.url));
const repoRoot = join(root, "..", "..");

test("node bindings smoke test", async (t) => {
  let dtcs;
  try {
    dtcs = await import(join(root, "index.js"));
  } catch (err) {
    t.skip(`node binding unavailable: ${err}`);
    return;
  }

  const fixture = readFileSync(
    join(repoRoot, "tests", "fixtures", "valid_customer.yaml"),
  );
  const parsed = dtcs.parseDocument(fixture, "yaml");
  assert.ok(parsed.contract);
  assert.equal(parsed.contract.id, "customer.normalize");
  assert.ok(parsed.report);

  const declaration = dtcs.conformanceDeclare();
  assert.equal(declaration.primaryProfile, "integrated-platform");
});
