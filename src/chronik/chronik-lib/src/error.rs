// Copyright (c) 2022 The Bitcoin developers
// Distributed under the MIT software license, see the accompanying
// file COPYING or http://www.opensource.org/licenses/mit-license.php.

use abc_rust_error::Result;
use chronik_bridge::ffi::ChronikBridge;
use chronik_util::log_chronik;

/// If `result` is [`Err`], logs and aborts the node.
pub(crate) fn ok_or_abort_node<T>(
    bridge: &ChronikBridge,
    func_name: &str,
    result: Result<T>,
) {
    if let Err(report) = result {
        log_chronik!("{report:?}\n");
        bridge.abort_node(
            &format!("ERROR Chronik in {func_name}"),
            &format!("{report:#}"),
        );
    }
}
