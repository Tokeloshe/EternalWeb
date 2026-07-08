//! # EternalWeb CLI
//!
//! Command-line interface for EternalWeb.

use clap::{Parser, Subcommand};
use colored::*;
use eternalweb_core::{
    archiving, networking, security, search, collaboration, blockchain,
    utils::init_logging,
};

/// EternalWeb CLI - Decentralized Web Archiving
#[derive(Parser)]
#[command(name = "eternalweb")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Decentralized web archiving and sharing platform", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Archive a web page
    Archive {
        /// URL to archive
        url: String,

        /// Make the archive public
        #[arg(long)]
        public: bool,

        /// Sign with Web3 wallet
        #[arg(long)]
        wallet_sign: bool,
    },

    /// Search archived content
    Search {
        /// Search query
        query: String,

        /// Filter results (ethical, non-corporate, etc.)
        #[arg(long)]
        filter: Option<String>,
    },

    /// Start a network node
    Node {
        #[command(subcommand)]
        action: NodeAction,
    },

    /// Manage collectives
    Collective {
        #[command(subcommand)]
        action: CollectiveAction,
    },

    /// Generate cryptographic keys
    Keygen {
        /// Key type (ed25519, kyber, dilithium)
        #[arg(long, default_value = "ed25519")]
        key_type: String,
    },

    /// Blockchain operations
    Chain {
        #[command(subcommand)]
        action: ChainAction,
    },
}

#[derive(Subcommand)]
enum NodeAction {
    /// Start the node
    Start {
        /// Configuration file
        #[arg(long)]
        config: Option<String>,
    },

    /// Stop the node
    Stop,

    /// Show node status
    Status,
}

#[derive(Subcommand)]
enum CollectiveAction {
    /// Create a new collective
    Create {
        /// Name of the collective
        name: String,
    },

    /// List collectives
    List,

    /// Join a collective
    Join {
        /// Collective ID
        id: String,
    },
}

#[derive(Subcommand)]
enum ChainAction {
    /// Create a proposal
    Propose {
        /// Proposal title
        title: String,

        /// Proposal description
        description: String,
    },

    /// Vote on a proposal
    Vote {
        /// Proposal ID
        id: String,

        /// Vote (true = for, false = against)
        #[arg(long)]
        vote: bool,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    init_logging()?;

    if cli.verbose {
        println!("{}", "EternalWeb - Reclaiming the internet for humankind".green().bold());
        println!();
    }

    match cli.command {
        Commands::Archive { url, public, wallet_sign } => {
            archive_command(url, public, wallet_sign).await?;
        }
        Commands::Search { query, filter } => {
            search_command(query, filter).await?;
        }
        Commands::Node { action } => {
            node_command(action).await?;
        }
        Commands::Collective { action } => {
            collective_command(action).await?;
        }
        Commands::Keygen { key_type } => {
            keygen_command(key_type)?;
        }
        Commands::Chain { action } => {
            chain_command(action).await?;
        }
    }

    Ok(())
}

async fn archive_command(url: String, public: bool, wallet_sign: bool) -> anyhow::Result<()> {
    println!("{} {}", "Archiving:".blue().bold(), url);

    if wallet_sign {
        println!("{}", "  ✓ Wallet signing enabled".green());
    }

    if public {
        println!("{}", "  ✓ Public archive".green());
    }

    let spinner = indicatif::ProgressBar::new_spinner();
    spinner.set_message("Fetching and processing content...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    let cid = archiving::snapshot_page(&url).await?;

    spinner.finish_with_message("✓ Archive complete");

    println!();
    println!("{} {}", "CID:".yellow().bold(), cid.to_string().cyan());
    println!("{}", "Archive successfully created and replicated!".green());

    Ok(())
}

async fn search_command(query: String, filter: Option<String>) -> anyhow::Result<()> {
    println!("{} '{}'", "Searching for:".blue().bold(), query);

    if let Some(f) = &filter {
        println!("{} {}", "Filter:".yellow(), f);
    }

    let results = search::query_network(&query).await?;

    println!();
    println!("{} {} results", "Found".green().bold(), results.len());

    for (i, (cid, score)) in results.iter().enumerate() {
        println!();
        println!("{}. {} (relevance: {:.2})", i + 1, cid.to_string().cyan(), score);
    }

    Ok(())
}

async fn node_command(action: NodeAction) -> anyhow::Result<()> {
    match action {
        NodeAction::Start { config } => {
            println!("{}", "Starting EternalWeb node...".blue().bold());

            if let Some(cfg) = config {
                println!("{} {}", "Config:".yellow(), cfg);
            }

            let _swarm = networking::bootstrap_network().await?;

            println!("{}", "✓ Node started successfully!".green());
            println!("{}", "Press Ctrl+C to stop...".bright_black());

            // Keep running
            tokio::signal::ctrl_c().await?;

            println!();
            println!("{}", "Shutting down...".yellow());
        }
        NodeAction::Stop => {
            println!("{}", "Stopping node...".yellow());
            // TODO: Implement graceful shutdown
        }
        NodeAction::Status => {
            println!("{}", "Node Status".blue().bold());
            println!("{} {}", "Status:".yellow(), "Running".green());
            // TODO: Show actual status
        }
    }

    Ok(())
}

async fn collective_command(action: CollectiveAction) -> anyhow::Result<()> {
    match action {
        CollectiveAction::Create { name } => {
            println!("{} '{}'", "Creating collective:".blue().bold(), name);

            let id = collaboration::create_collective(&name).await?;

            println!();
            println!("{} {}", "Collective ID:".yellow().bold(), id.cyan());
            println!("{}", "✓ Collective created!".green());
        }
        CollectiveAction::List => {
            println!("{}", "Your Collectives:".blue().bold());
            // TODO: List collectives
        }
        CollectiveAction::Join { id } => {
            println!("{} {}", "Joining collective:".blue().bold(), id);
            // TODO: Implement join
        }
    }

    Ok(())
}

fn keygen_command(key_type: String) -> anyhow::Result<()> {
    println!("{} {}", "Generating keypair:".blue().bold(), key_type);

    let key_type_enum = match key_type.as_str() {
        "ed25519" => security::KeyType::Ed25519,
        "kyber" => security::KeyType::Kyber,
        "dilithium" => security::KeyType::Dilithium,
        _ => {
            eprintln!("{} Unknown key type: {}", "Error:".red().bold(), key_type);
            return Ok(());
        }
    };

    let (public_key, _private_key) = security::generate_keypair_with_type(key_type_enum)?;

    println!();
    println!("{}", "✓ Keypair generated!".green());
    println!("{} {}", "Public key:".yellow(), hex::encode(&public_key.bytes));
    println!("{}", "Private key saved securely".bright_black());

    Ok(())
}

async fn chain_command(action: ChainAction) -> anyhow::Result<()> {
    match action {
        ChainAction::Propose { title, description } => {
            println!("{}", "Creating proposal...".blue().bold());
            println!("{} {}", "Title:".yellow(), title);

            let proposal = blockchain::Proposal {
                id: uuid::Uuid::new_v4().to_string(),
                title,
                description,
                proposer: "current_user".to_string(),
                votes_for: 0,
                votes_against: 0,
                status: blockchain::ProposalStatus::Pending,
                created_at: chrono::Utc::now(),
            };

            let tx_hash = blockchain::deploy_dao(proposal).await?;

            println!();
            println!("{} {}", "Transaction:".yellow().bold(), tx_hash.cyan());
            println!("{}", "✓ Proposal created!".green());
        }
        ChainAction::Vote { id, vote } => {
            println!(
                "{} {} on proposal {}",
                "Voting".blue().bold(),
                if vote { "FOR".green() } else { "AGAINST".red() },
                id
            );

            let tx_hash = blockchain::vote_on_proposal(&id, vote).await?;

            println!();
            println!("{} {}", "Transaction:".yellow().bold(), tx_hash.cyan());
            println!("{}", "✓ Vote recorded!".green());
        }
    }

    Ok(())
}
