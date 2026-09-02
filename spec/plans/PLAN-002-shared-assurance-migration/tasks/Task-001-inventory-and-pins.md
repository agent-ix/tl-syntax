---
id: Task-001
title: "Inventory and pins"
type: Task
status: done
track: Migration
priority: P0
relationships:
  - target: ix://agent-ix/tl-syntax/PLAN-002
    type: part_of
  - target: ix://agent-ix/tl-syntax/FR-006
    type: references
---
# Task-001: Inventory and pins

## Scope

Separate generic assurance machinery from domain verification behaviour, and pin
the accepted shared release without restating its compatibility matrix.

## Completion Evidence

The keep/replace/delete/defer inventory is recorded in the plan overview and is
the basis for the deletion commit. `requirements-assurance.txt` pins
`engineering-assurance` at the `v0.2.0` tag, which is the released artifact and
not a branch head. `assurance/pins.json` records the digests of the four
artifacts this repository reads out of that release and deliberately records no
component versions, because the packaged matrix is their authority.

`scripts/check_shared_pins.py` observes the local toolchain and delegates every
verdict to `engineering_assurance.compatibility`. It reports the acceptance state
the release carries and gates only on what is local: version classification,
consumed-artifact digests, and the absence of any internal mirror registry
reference.
