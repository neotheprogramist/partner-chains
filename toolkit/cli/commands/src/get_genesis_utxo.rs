use sidechain_domain::UtxoId;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::Block as BlockT;
use sp_sidechain::GetGenesisUtxo;
use std::sync::Arc;

/// # Genesis UTXO
/// 
/// Retrieve the genesis UTXO from on-chain storage.
///
/// Queries the Partner Chain runtime API to fetch the genesis UTXO that uniquely
/// identifies the Partner Chain instance. The genesis UTXO is stored in on-chain
/// storage and accessible through the runtime API.
/// 
/// ## Process Overview
///
/// 1. Connect to Partner Chain runtime API
/// 2. Query genesis UTXO from on-chain storage
/// 3. Format output as JSON with genesis UTXO information
/// 
/// ## CLI Integration
/// 
/// ```bash
/// cargo run --bin partner-chains-demo-node -- sidechain-params <OPTIONS>
/// ```
/// In options specify the chain specification. It can be one of the predefined ones (dev, local, or staging) or it can be a path to a file with the chainspec (such as one exported by the `build-spec` subcommand).
///
/// ## Output Format
/// 
/// ```json
/// {
///   "genesis_utxo": "e41c9b57841e582c207bb68d5e9736fb48c7af5f1ec29ade00692fa5e0e47efa#4"
/// }
/// ```

pub async fn execute<B, C>(client: Arc<C>) -> Result<String, String>
where
	B: BlockT,
	C: ProvideRuntimeApi<B> + Send + Sync + 'static,
	C::Api: GetGenesisUtxo<B>,
	C: HeaderBackend<B>,
{
	let api = client.runtime_api();
	let best_block = client.info().best_hash;
	let genesis_utxo = api.genesis_utxo(best_block).map_err(|err| err.to_string())?;
	let output =
		serde_json::to_string_pretty(&Output { genesis_utxo }).map_err(|err| err.to_string())?;
	Ok(output)
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
struct Output {
	pub genesis_utxo: UtxoId,
}
