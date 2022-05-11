pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

pub fn print_built_info() {
    let branch = match built_info::GIT_HEAD_REF {
        Some(r) => r,
        None => "unknown",
    };
    let commit = match built_info::GIT_COMMIT_HASH {
        Some(r) => r,
        None => "unknown",
    };
    tracing::info!(
        "Built info:
    Name: {}
    Author: {}
    Version: {}
    Branch: {}
    Commit: {}
    Build: {}
    OS: {}
    Family: {}
    Arch: {} {}-endian
    Profile: {}",
        built_info::PKG_NAME,
        built_info::PKG_AUTHORS,
        built_info::PKG_VERSION,
        branch,
        commit,
        built_info::RUSTC_VERSION,
        built_info::CFG_OS,
        built_info::CFG_FAMILY,
        built_info::CFG_TARGET_ARCH,
        built_info::CFG_ENDIAN,
        built_info::PROFILE,
    );
}

