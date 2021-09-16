extern crate prettytable;

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{PKG_VERSION, model::AttributeValue, Client, Error, Region};
use prettytable::{color, Attr, Cell, Row, Table};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(short, long)]
    // AWS Region (ex: us-east-2)
    region: Option<String>,
    // DynamoDB Table name
    #[structopt(short, long)]
    table: String,
    // Shows additional Info if --i or --info is provided
    #[structopt(short, long)]
    info: bool,
}

// Main
#[tokio::main()]
async fn main() -> Result<(), Error> {
    //Enable Tracing
    tracing_subscriber::fmt::init();

    // Get command line args
    let Options {
        region,
        table,
        info,
    } = Options::from_args();

    // Get AWS credentials and Region info
    let region_provider = RegionProviderChain::first_try(region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-east-2"));

    // If AWS_REGION not present in region provider, try loading from ENVIRONMENT Variables
    let shared_config = aws_config::from_env().region(region_provider).load().await;

    // Initialize a new AWS DynamoDB Client
    let client = Client::new(&shared_config);

    let mut info_table = Table::new();

    // If Info flag is supplied, show additional info
    if info {
        info_table.add_row(Row::new(vec![
            Cell::new("DynamoDB Client Version").with_style(Attr::ForegroundColor(color::RED)),
            Cell::new("Region").with_style(Attr::ForegroundColor(color::YELLOW)),
            Cell::new("Table Name").with_style(Attr::ForegroundColor(color::GREEN)),
        ]));

        info_table.add_row(Row::new(vec![
            Cell::new(PKG_VERSION).with_style(Attr::ForegroundColor(color::RED)),
            Cell::new(&shared_config.region().unwrap().to_string())
                .with_style(Attr::ForegroundColor(color::YELLOW)),
            Cell::new(&table).with_style(Attr::ForegroundColor(color::GREEN)),
        ]));

        println!();

        info_table.printstd();
    }

    // Scan the table for all items
    let response = client.scan().table_name(table).send().await?;

    // Format and display tbe table
    if let Some(item) = response.items {
        let mut keys: Vec<String> = Vec::new();
        let mut pty_table = Table::new();
        let mut idx = 0;

        for hm in item {
            if idx == 0 {
                keys = hm.keys().cloned().into_iter().collect::<Vec<String>>();

                keys.sort_by(|a, b| b.cmp(&a));

                let header = hm
                    .keys()
                    .cloned()
                    .into_iter()
                    .map(|k| {
                        Cell::new(&k.to_ascii_uppercase())
                            .with_style(Attr::Blink)
                            .with_style(Attr::ForegroundColor(color::BRIGHT_MAGENTA))
                    })
                    .collect::<Vec<Cell>>();

                pty_table.add_row(Row::new(header));
            }

            let mut values = Vec::new();
            //get keys
            //get values from those keys
            for k in &keys {
                if let Some(attrib) = hm.get(k) {
                    match attrib {
                        AttributeValue::S(n) => { // Match string fields 
                            if n.len() > 50 {
                                values.push(
                                    Cell::new(&n[..50])
                                        .with_style(Attr::ForegroundColor(color::BLUE)),
                                );
                            } else {
                                values.push(
                                    Cell::new(n).with_style(Attr::ForegroundColor(color::BLUE)),
                                );
                            }
                        }
                        AttributeValue::N(n) => values  // Match Number fields
                            .push(Cell::new(n).with_style(Attr::ForegroundColor(color::GREEN))),
                        _ => values.push(Cell::new("N/A")), // Other data types are not supported in this demo
                    }
                }
            }
            pty_table.add_row(Row::new(values));

            idx += 1;
        }

        println!();

        // Print the main table
        pty_table.printstd();
    }

    Ok(())
}
