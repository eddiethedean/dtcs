import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
import test from "node:test";

const root = dirname(fileURLToPath(import.meta.url));
const pkgRoot = join(root, "..", "pkg");
const fixtures = join(root, "..", "..", "..", "tests", "fixtures");

function mapToPlain(value) {
  if (value instanceof Map) {
    return Object.fromEntries(
      [...value.entries()].map(([key, entry]) => [key, mapToPlain(entry)]),
    );
  }
  if (Array.isArray(value)) {
    return value.map(mapToPlain);
  }
  return value;
}

function mapGet(value, key) {
  return value instanceof Map ? value.get(key) : value[key];
}

function diagnosticIds(report) {
  return report.diagnostics.map((d) => d.id).sort();
}

function isValidReport(report) {
  return !report.diagnostics.some((d) => d.severity === "error");
}

test("wasm bindings smoke test", async (t) => {
  let wasm;
  try {
    wasm = await import(join(pkgRoot, "dtcs_wasm.js"));
  } catch (err) {
    if (process.env.CI || process.env.DTCS_REQUIRE_BINDINGS === "1") {
      throw err;
    }
    t.skip(`wasm package not built: ${err}`);
    return;
  }

  wasm.initSync({ module: readFileSync(join(pkgRoot, "dtcs_wasm_bg.wasm")) });
  assert.equal(wasm.specVersion(), "3.0.0");

  const customerParsed = wasm.parseDocument(
    readFileSync(join(fixtures, "valid_customer.yaml")),
    "yaml",
  );
  const customerContract = mapToPlain(mapGet(customerParsed, "contract"));
  assert.equal(customerContract.id, "customer.normalize");
  assert.ok(mapGet(customerParsed, "report"));
  assert.ok(isValidReport(wasm.validateContract(customerContract)));

  const minimalParsed = wasm.parseDocument(
    readFileSync(join(fixtures, "valid_minimal.yaml")),
    "yaml",
  );
  const minimalContract = mapToPlain(mapGet(minimalParsed, "contract"));
  assert.ok(isValidReport(wasm.validateContract(minimalContract)));
  assert.equal(minimalContract.id, "json.example");

  const missingParsed = wasm.parseDocument(
    readFileSync(join(fixtures, "missing_lineage.yaml")),
    "yaml",
  );
  const missingValidated = wasm.validateContract(
    mapToPlain(mapGet(missingParsed, "contract")),
  );
  assert.deepEqual(diagnosticIds(missingValidated), [
    "dtcs:ambiguous-reference",
    "dtcs:missing-lineage",
  ]);

  const declaration = wasm.conformanceDeclare();
  assert.equal(declaration.primaryProfile, "integrated-platform");
  assert.equal(declaration.profiles.length, 23);
});
