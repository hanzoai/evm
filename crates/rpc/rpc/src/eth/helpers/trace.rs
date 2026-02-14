//! Contains RPC handler implementations specific to tracing.

use hanzo_evm_rpc_convert::RpcConvert;
use hanzo_evm_rpc_eth_api::{helpers::Trace, FromEvmError, RpcNodeCore};
use hanzo_evm_rpc_eth_types::EthApiError;

use crate::EthApi;

impl<N, Rpc> Trace for EthApi<N, Rpc>
where
    N: RpcNodeCore,
    EthApiError: FromEvmError<N::Evm>,
    Rpc: RpcConvert<Primitives = N::Primitives, Error = EthApiError, Evm = N::Evm>,
{
}
