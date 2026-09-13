---
id: Task-003
title: Verify the corpus and close the review
type: Task
status: in_progress
relationships:
  - target: ix://agent-ix/tl-syntax/PLAN-009
    type: part_of
---

# Task-003: Verify the corpus and close the review

Run the complete local gate at the exact final head. At PR time, run the Rust
review and gap analysis, address every finding, and re-run the gate at the new
head. Hosted CI remains manual-only.
