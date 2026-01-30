;; SPDX-License-Identifier: PMPL-1.0-or-later
;; ECOSYSTEM.scm - Ecosystem relationships for czech-file-knife
;; Media type: application/vnd.ecosystem+scm

(ecosystem
  (metadata
    ((version . "1.0.0")
     (name . "czech-file-knife")
     (type . "file-tool")
     (purpose . "Part of hyperpolymath tool ecosystem")))
  
  (position-in-ecosystem
    "Provides file-tool functionality within the hyperpolymath suite")
  
  (related-projects
    ((bunsenite . "sibling-tool")
     (vext . "sibling-tool")))
  
  (what-this-is
    "czech-file-knife is a specialized tool in the hyperpolymath ecosystem")
  
  (what-this-is-not
    "Not a general-purpose framework"
    "Not intended as standalone product"))
