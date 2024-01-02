use chrono::{Utc, DateTime, Duration};
use splitwise::model::expenses::ListExpensesRequest;

const FAMILY_GROUP_ID : i64 = 42513339;

fn get_past_day_midnight() -> DateTime<Utc> {
    let midnight = Utc::now().date().and_hms_opt(0,0,0).unwrap();
    println!("Current date {:#?}", midnight);
    midnight
}

fn build_expenses_request(group_id: i64) -> ListExpensesRequest {
    let pd_midnight = get_past_day_midnight();
    let nd_midnight = (pd_midnight + Duration::days(1)).date().and_hms_opt(0,0,0).unwrap();
    ListExpensesRequest {
        group_id: Some(group_id),
        friend_id: None,
        dated_after: Some(pd_midnight),
        dated_before: Some(nd_midnight),
        updated_after: None,
        updated_before: None,
        limit: None,
        offset: None,
    }
}

#[tokio::main]
async fn main() {
    let client = splitwise::client::Client::default();
    // let groups = client.groups().list_groups().await.unwrap();
    // println!("Current groups: {:#?}", groups);
    let expenses = client.expenses().list_expenses(build_expenses_request(FAMILY_GROUP_ID)).await.unwrap();
    println!("Current expenses: {:#?}", expenses);
}
