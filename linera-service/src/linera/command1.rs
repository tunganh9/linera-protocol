        /// The number of tokens to send to each new chain created by the faucet.
         #[arg(long, default_value = "1000")]
         faucet_amount: Amount,
 
         /// The number of block exporters per validator in the local test network. Default is 0.
         #[arg(long, default_value = "0")]
         block_exporters: u32,
     },
 
     /// Print a bash helper script to make `linera net up` easier to use. The script is
     /// meant to be installed in `~/.bash_profile` or sourced when needed.
     Helper,
 }
 
 #[derive(Clone, clap::Subcommand)]
 pub enum WalletCommand {
     /// Show the contents of the wallet.
     Show {
         /// The chain to show the metadata.
         chain_id: Option<ChainId>,
         /// Only print a non-formatted list of the wallet's chain IDs.
         #[arg(long)]
         short: bool,
         /// Print only the chains that we have a key pair for.
         #[arg(long)]
         owned: bool,
     },
 
     /// Change the wallet default chain.
     SetDefault { chain_id: ChainId },
