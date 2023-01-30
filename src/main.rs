fn main() {
    let start_amount = 100;
    let transactions: Vec<Vec<String>> = vec![
        vec!["BUY".to_string(), "Apple".to_string(), "40".to_string()],
        vec!["BUY".to_string(), "SoundMind".to_string(), "40".to_string()],
        vec!["SELL".to_string(), "Apple".to_string(), "40".to_string()],
        vec!["BUY".to_string(), "Tesla".to_string(), "40".to_string()],
        vec!["BUY".to_string(), "Google".to_string(), "60".to_string()],
        vec!["BUY".to_string(), "Google".to_string(), "90".to_string()],
        vec!["SELL".to_string(), "Google".to_string(), "90".to_string()],
    ];

    let transactions_2: Vec<Vec<String>> = vec![
        vec!["BUY".to_string(), "Apple".to_string(), "40".to_string()],
        vec!["BUY".to_string(), "SoundMind".to_string(), "50".to_string()],
        vec!["BUY".to_string(), "RobinHood".to_string(), "30".to_string()],
        vec![
            "BUY".to_string(),
            "Inked Sports".to_string(),
            "120".to_string(),
        ],
        vec!["SELL".to_string(), "Apple".to_string(), "40".to_string()],
        vec!["BUY".to_string(), "Google".to_string(), "10".to_string()],
    ];
    println!("{:?}", invest(transactions, start_amount));
    println!("{:?}", invest(transactions_2, start_amount));
}

fn invest(transactions: Vec<Vec<String>>, start_amount: i32) -> Vec<Vec<String>> {
    let mut final_portfolio: Vec<Vec<String>> = vec![];
    let mut money: u32 = start_amount as u32;
    for trans in transactions {
        let trans_action = &trans[0];
        let trans_company = &trans[1];
        let trans_amount: i32 = trans[2].parse::<i32>().unwrap();
        // Option 1: BUY
        if trans_action == "BUY" {
            // Case 1: Enough money to buy
            if money as i32 >= trans_amount {
                final_portfolio.push(vec![trans_company.to_string(), trans_amount.to_string()]);
                money -= trans_amount as u32;
            } else {
                // Case 2: Not enough money to buy
                // Case: Only perform operation if transaction amount is less than total starting money
                if trans_amount <= 100 {
                    let mut amount_left_to_add = trans_amount - money as i32;
                    while amount_left_to_add != 0 {
                        let portfolio_length = final_portfolio.len();
                        let last_investment = &mut final_portfolio[portfolio_length - 1];
                        let last_investment_amount = last_investment[1].parse::<i32>().unwrap();
                        if amount_left_to_add >= last_investment_amount {
                            final_portfolio.pop();
                            amount_left_to_add -= last_investment_amount;
                        } else {
                            last_investment[1] =
                                (last_investment_amount - amount_left_to_add).to_string();
                            amount_left_to_add = 0;
                        }
                    }
                    final_portfolio.push(vec![trans_company.to_string(), trans_amount.to_string()]);
                    money = 0;
                }
            }
        } else {
            // Option 2: SELL
            let index_of_sell = final_portfolio
                .iter()
                .position(|p| &p[0] == trans_company)
                .unwrap();
            let portfolio_item = &mut final_portfolio[index_of_sell];
            if &portfolio_item[0] == trans_company {
                let investment_amount = portfolio_item[1].parse::<i32>().unwrap();
                // Case 1: Sell amount less than already bought amount
                if trans_amount < investment_amount {
                    portfolio_item[1] = (investment_amount - trans_amount).to_string();
                } else {
                    // Case 2: Sell amount greater than or equal to already bought amount, so have to delete from final portfolio
                    final_portfolio.remove(index_of_sell);
                    money += trans_amount as u32;
                }
            }
        }
    }
    if money != 0 {
        final_portfolio.push(vec!["CASH".to_string(), money.to_string()]);
    }
    return final_portfolio;
}
