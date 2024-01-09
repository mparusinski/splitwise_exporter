use std::{env,process};
use chrono::{Utc, DateTime, Duration};
use splitwise::model::expenses::ListExpensesRequest;

const APIKEY_ENVVAR : &'static str = "SPLITWISE_API_KEY";
const FAMILY_GROUP_NAME : &'static str = "Family 😍";

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
            let expenses = client.expenses().list_expenses(expense_req).await;
            expenses.map(| expenses_vec | {
                expenses_vec.iter().for_each(| an_expense | {
                    println!("{:#?}", an_expense);
                })
            });
        }
    }
}
