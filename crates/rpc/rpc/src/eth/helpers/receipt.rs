//! Builds an RPC receipt response w.r.t. data layout of network.

use crate::EthApi;
use hanzo_evm_rpc_convert::RpcConvert;
use hanzo_evm_rpc_eth_api::{helpers::LoadReceipt, FromEvmError, RpcNodeCore};
use hanzo_evm_rpc_eth_types::EthApiError;

impl<N, Rpc> LoadReceipt for EthApi<N, Rpc>
where
    N: RpcNodeCore,
    EthApiError: FromEvmError<N::Evm>,
    Rpc: RpcConvert<Primitives = N::Primitives, Error = EthApiError>,
{
}
