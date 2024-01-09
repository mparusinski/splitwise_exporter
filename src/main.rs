use std::{env,process};
use csv::Writer;
use chrono::{Utc, DateTime, Duration};
use splitwise::model::expenses::ListExpensesRequest;

const APIKEY_ENVVAR : &'static str = "SPLITWISE_API_KEY";
const FAMILY_GROUP_NAME : &'static str = "Family 😍";

#[derive(serde::Serialize)]
struct Row<'a> {
    description: &'a str,
    date: DateTime<Utc>,
    currency_code: &'a str,
    created_by: &'a str,
    category: &'a str,
}

fn get_past_day_midnight() -> DateTime<Utc> {
    let midnight = Utc::now().date().and_hms_opt(0,0,0).unwrap();
    println!("Current date {:#?}", midnight);
    midnight
}

fn build_expenses_request(group_id: i64) -> ListExpensesRequest {
    let pd = get_past_day_midnight();
    ListExpensesRequest {
        group_id: Some(group_id),
        friend_id: None,
        dated_after: None,
        dated_before: Some(pd),
        updated_after: None,
        updated_before: None,
        limit: None,
        offset: None,
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        let program = &args[0];
        println!("Usage: {program} output.csv");
        process::exit(-1);
    }
    let output_file = &args[1];
    println!("Output CSV file {output_file}");

    match env::var(APIKEY_ENVVAR) {
        Ok(val) => println!("{APIKEY_ENVVAR} defined => All is good!"),
        Err(e) => {
            println!("{APIKEY_ENVVAR} wrongly defined => {e}");
            process::exit(1);
        },
    }
    let client = splitwise::client::Client::default();
    let groups = client.groups().list_groups().await;
    let mut maybe_group_id : Option<i64> = None;
    groups.map(| group_vec | {
        group_vec.iter().for_each(| a_group | {
            match &a_group.name {
                None => (),
                Some(group_name) => {
                    if (group_name == FAMILY_GROUP_NAME) {
                        maybe_group_id = a_group.id;
                    }
                }
            }
        });
    });

    match maybe_group_id {
        None => {
            println!("Didn't find group \"{FAMILY_GROUP_NAME}\" amongst groups");
            process::exit(2);
        },
        Some(group_id) => {
            println!("Found {FAMILY_GROUP_NAME} with group id {group_id}");
            let expense_req = build_expenses_request(group_id);
            let mut expenses = client.expenses().list_expenses(expense_req).await;
            expenses.map(| expenses_vec | {
                let mut wtr = Writer::from_path(output_file).unwrap();
                let default_date = Utc::now().date().and_hms_opt(0,0,0).unwrap();
                expenses_vec.iter().for_each(| e | {
                    // println!("{:#?}", e);
                    wtr.serialize(Row {
                        // TODO: Find a way to merge two optionals (must be possible)
                        description: e.description.clone().unwrap_or_default().as_str(),
                        date: e.date.clone().unwrap_or_default(),
                        currency_code: e.currency_code.clone().unwrap_or_default().as_str(),
                        created_by: e.created_by.clone().map(| c | { 
                            c.first_name
                        }).unwrap().unwrap_or_default().as_str(),
                        category: e.category.clone().map(| c | { 
                            c.name
                        }).unwrap().unwrap_or_default().as_str(),
                    }).unwrap();
                });
                wtr.flush();
            });
        }
    }
}
