import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
import test from "node:test";

const root = dirname(fileURLToPath(import.meta.url));
const repoRoot = join(root, "..", "..", "..");
const fixtures = join(repoRoot, "tests", "fixtures");

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

test("node bindings smoke test", async (t) => {
  let dtcs;
  try {
    dtcs = await import(join(root, "..", "index.js"));
  } catch (err) {
    t.skip(`node binding unavailable: ${err}`);
    return;
  }

  const customerParsed = dtcs.parseDocument(
    readFileSync(join(fixtures, "valid_customer.yaml")),
    "yaml",
  );
  const customerContract = mapToPlain(mapGet(customerParsed, "contract"));
  assert.equal(customerContract.id, "customer.normalize");
  assert.ok(mapGet(customerParsed, "report"));
  assert.ok(isValidReport(dtcs.validateContract(customerContract)));

  const minimalParsed = dtcs.parseDocument(
    readFileSync(join(fixtures, "valid_minimal.yaml")),
    "yaml",
  );
  const minimalContract = mapToPlain(mapGet(minimalParsed, "contract"));
  assert.ok(isValidReport(dtcs.validateContract(minimalContract)));
  assert.equal(minimalContract.id, "json.example");

  const missingParsed = dtcs.parseDocument(
    readFileSync(join(fixtures, "missing_lineage.yaml")),
    "yaml",
  );
  const missingValidated = dtcs.validateContract(
    mapToPlain(mapGet(missingParsed, "contract")),
  );
  assert.deepEqual(diagnosticIds(missingValidated), [
    "dtcs:ambiguous-reference",
    "dtcs:missing-lineage",
  ]);

  const declaration = dtcs.conformanceDeclare();
  assert.equal(declaration.primaryProfile, "integrated-platform");
  assert.equal(declaration.profiles.length, 8);
});
