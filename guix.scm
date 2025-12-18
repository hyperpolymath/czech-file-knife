;; czech-file-knife - Guix Package Definition
;; SPDX-License-Identifier: AGPL-3.0-or-later
;; SPDX-FileCopyrightText: 2025 Jonathan D.A. Jewell
;;
;; Run: guix shell -D -f guix.scm
;; Build: guix build -f guix.scm

(use-modules (guix packages)
             (guix gexp)
             (guix git-download)
             (guix build-system cargo)
             ((guix licenses) #:prefix license:)
             (gnu packages crates-io)
             (gnu packages crates-graphics)
             (gnu packages rust)
             (gnu packages rust-apps)
             (gnu packages linux)
             (gnu packages pkg-config))

(define-public czech-file-knife
  (package
    (name "czech-file-knife")
    (version "0.1.0")
    (source (local-file "." "czech-file-knife-checkout"
                        #:recursive? #t
                        #:select? (git-predicate ".")))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-build-flags '("--release" "-p" "cfk-cli")
       #:install-source? #f
       #:phases
       (modify-phases %standard-phases
         (add-after 'install 'install-binary
           (lambda* (#:key outputs #:allow-other-keys)
             (let* ((out (assoc-ref outputs "out"))
                    (bin (string-append out "/bin")))
               (mkdir-p bin)
               (install-file "target/release/cfk" bin)))))))
    (native-inputs
     (list pkg-config rust))
    (inputs
     (list fuse-3))
    (synopsis "Cloud-native Swiss File Knife - unified interface for 20+ storage backends")
    (description
     "Czech File Knife (CFK) is a cloud-native Swiss File Knife providing a
unified interface for 20+ storage backends including local filesystem, S3,
SFTP, WebDAV, cloud storage (Dropbox, Google Drive, OneDrive), and distributed
filesystems (IPFS, Syncthing).  Features include FUSE mounting, full-text
search with Tantivy, offline caching, and integrations with external tools
like aria2, agrep, pandoc, and OCR engines.")
    (home-page "https://github.com/hyperpolymath/czech-file-knife")
    (license license:agpl3+)))

;; Return package for guix shell
czech-file-knife
