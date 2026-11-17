// Integration test for SigmaOS-emacs-3d-stencil-buffer-theme-directory
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use sigmaos_emacs_3d_stencil_buffer_theme_directory as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
