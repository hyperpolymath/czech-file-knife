;; SPDX-License-Identifier: AGPL-3.0-or-later
;; SPDX-FileCopyrightText: 2025 Jonathan D.A. Jewell
;; ECOSYSTEM.scm — czech-file-knife

(ecosystem
  (version "1.0.0")
  (name "czech-file-knife")
  (type "project")
  (purpose "*Cloud-native Swiss File Knife - unified interface for 20+ storage backends*")

  (position-in-ecosystem
    "Part of hyperpolymath ecosystem. Follows RSR guidelines.")

  (related-projects
    (project (name "rhodium-standard-repositories")
             (url "https://github.com/hyperpolymath/rhodium-standard-repositories")
             (relationship "standard")))

  (what-this-is "*Cloud-native Swiss File Knife - unified interface for 20+ storage backends*")
  (what-this-is-not "- NOT exempt from RSR compliance"))
