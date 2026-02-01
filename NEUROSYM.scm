; SPDX-License-Identifier: PMPL-1.0-or-later
; NEUROSYM.scm - Neurosymbolic context for czech-file-knife
; Media type: application/vnd.neurosym+scm

(neurosym
  (metadata
    (version "1.0.0")
    (schema-version "1.0")
    (created "2026-01-30")
    (updated "2026-01-30"))

  (conceptual-model
    (domain "file-management")
    (subdomain "automation")
    (core-concepts
      (concept "tool"
        (definition "A software component that automates tasks")
        (properties "input" "output" "configuration"))))

  (knowledge-graph-hints
    (entities "czech-file-knife" "Rust" "automation")
    (relationships
      ("czech-file-knife" provides "automation-capabilities"))))
