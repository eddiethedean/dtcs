# How Cursor Should Use SPEC.md

`SPEC.md` is authoritative.

Cursor should:

1. Read relevant sections of `SPEC.md` before implementing each module.
2. Preserve terminology exactly as defined in `SPEC.md`.
3. Prefer spec-aligned names over ad hoc names.
4. Treat examples in this pack as illustrative only.
5. Treat conflicts between this pack and `SPEC.md` as resolved in favor of `SPEC.md`.

Do not invent behavior that is not supported by `SPEC.md`.

If the spec is ambiguous, implement the smallest conservative behavior and add a TODO referencing the relevant section.
