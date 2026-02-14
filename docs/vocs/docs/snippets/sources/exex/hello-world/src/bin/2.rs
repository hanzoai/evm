use evm::api::FullNodeComponents;
use hanzo_evm_exex::ExExContext;
use hanzo_evm_node_ethereum::EthereumNode;

async fn my_exex<Node: FullNodeComponents>(mut _ctx: ExExContext<Node>) -> eyre::Result<()> {
    #[expect(clippy::empty_loop)]
    loop {}
}

fn main() -> eyre::Result<()> {
    reth::cli::Cli::parse_args().run(async move |builder, _| {
        let handle = builder
            .node(EthereumNode::default())
            .install_exex("my-exex", async move |ctx| Ok(my_exex(ctx)))
            .launch_with_debug_capabilities()
            .await?;

        handle.wait_for_node_exit().await
    })
}
