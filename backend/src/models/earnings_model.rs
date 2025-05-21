use crate::calculate;
use crate::query;
use crate::schema::nominal_earnings;
use bullseye_api::table::ConcatStatement;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = nominal_earnings)]
pub struct NominalEarnings {
    id: i32,
    exchange: String,
    ticker: String,
    duration: String,
    pub quarter_str: i16,
    year_str: i16,
    pub revenue: f64,
    pub revenue_growth_yoy: f64,
    cost_of_revenue: f64,
    gross_profit: f64,
    pub gross_margin: f64,
    pub gross_profit_growth_yoy: Option<f64>,
    sga_expenses: Option<f64>,
    pub sga_gp_ratio: Option<f64>,
    rnd_expenses: Option<f64>,
    pub rnd_gp_ratio: Option<f64>,
    operating_expenses: f64,
    operating_income: f64,
    pub operating_margin: f64,
    interest_expenses: Option<f64>,
    pub interest_expenses_op_income_ratio: Option<f64>,
    pub net_income: f64,
    pub net_margin: f64,
    eps_basic: f64,
    eps_diluted: f64,
    shares_outstanding_basic: f64,
    pub shares_outstanding_diluted: f64,
    pub shares_change_yoy: f64,
    cash_and_equivalents: f64,
    cash_and_short_term_investments: f64,
    accounts_receivable: Option<f64>,
    inventory: Option<f64>,
    total_current_assets: f64,
    goodwill: Option<f64>,
    total_assets: f64,
    accounts_payable: Option<f64>,
    total_current_liabilities: f64,
    total_liabilities: f64,
    pub retained_earnings: f64,
    shareholders_equity: f64,
    total_debt: Option<f64>,
    pub net_cash: f64,
    depreciation_and_amortization: Option<f64>,
    stock_based_compensation: Option<f64>,
    pub operating_cash_flow: Option<f64>,
    pub operating_cash_flow_margin: Option<f64>,
    capital_expenditure: Option<f64>,
    investing_cash_flow: Option<f64>,
    financing_cash_flow: Option<f64>,
    pub free_cash_flow: Option<f64>,
    pub free_cash_flow_margin: Option<f64>,
    ratio_calculated: bool,
    growth_calculated: bool,
}
impl NominalEarnings {
    /// retrieves the lastest quarterly(TTM) earnings data for the given ticker
    pub fn latest_quarter_data(
        target_ticker: &str,
        target_exchange: &str,
        conn: &mut PgConnection,
    ) -> Result<Self, DieselError> {
        use crate::schema::nominal_earnings::dsl::*;
        query::load_first_row(
            nominal_earnings
                .filter(ticker.eq(target_ticker))
                .filter(exchange.eq(target_exchange))
                .filter(duration.eq("T"))
                .order((year_str.desc(), quarter_str.desc())),
            conn,
        )
    }

    /// retrieves the lastest annual earnings data for the given ticker
    pub fn latest_annual_data(
        target_ticker: &str,
        target_exchange: &str,
        conn: &mut PgConnection,
    ) -> Result<Self, DieselError> {
        use crate::schema::nominal_earnings::dsl::*;
        query::load_first_row(
            nominal_earnings
                .filter(ticker.eq(target_ticker))
                .filter(exchange.eq(target_exchange))
                .filter(duration.eq("Y"))
                .order(year_str.desc()),
            conn,
        )
    }

    /// retrieves the same quarter earnings data from the prvious year for the given ticker
    pub fn same_quarter_prev_year_data(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<Self>, DieselError> {
        use crate::schema::nominal_earnings::dsl::*;
        let prev_year_result = query::load_first_row(
            nominal_earnings
                .filter(exchange.eq(&self.exchange))
                .filter(ticker.eq(&self.ticker))
                .filter(duration.eq(&self.duration))
                .filter(year_str.eq(self.year_str - 1))
                .filter(quarter_str.eq(self.quarter_str)),
            conn,
        );
        let prev_year = match prev_year_result {
            Ok(data) => Ok(Some(data)),
            Err(DieselError::NotFound) => Ok(None),
            Err(e) => Err(e),
        };
        prev_year
    }

    /// updates missing ratios and margins for the selected earnings
    pub fn update_ratios(&self, conn: &mut PgConnection) -> Result<(), DieselError> {
        use crate::schema::nominal_earnings::dsl::*;
        let curr_id = self.id;
        let sga_ratio = calculate::calculate_ratio(self.sga_expenses, self.gross_profit);
        let rnd_ratio = calculate::calculate_ratio(self.rnd_expenses, self.gross_profit);
        let interest_ratio =
            calculate::calculate_ratio(self.interest_expenses, self.operating_income);
        let ocfm = calculate::calculate_ratio_as_pct(self.operating_cash_flow, self.revenue);
        query::update_earnings_table(
            curr_id,
            (
                sga_gp_ratio.eq(sga_ratio),
                rnd_gp_ratio.eq(rnd_ratio),
                interest_expenses_op_income_ratio.eq(interest_ratio),
                operating_cash_flow_margin.eq(ocfm),
                ratio_calculated.eq(true),
            ),
            conn,
        )?;
        Ok(())
    }

    /// updates missing growth rate for the selected earnings
    pub fn update_gp_yoy_growth(&self, conn: &mut PgConnection) -> Result<(), DieselError> {
        use crate::schema::nominal_earnings::dsl::*;
        let curr_id = self.id;
        let curr_gross_profit = self.gross_profit;
        let prev_year_data = self.same_quarter_prev_year_data(conn)?;
        let prev_gross_profit = prev_year_data.map(|data| data.gross_profit);
        if curr_gross_profit == 0. || prev_gross_profit.unwrap_or(0.) == 0. {
            query::update_earnings_table(curr_id, growth_calculated.eq(true), conn)?;
        } else {
            let gp_growth =
                calculate::calculate_yoy_growth(curr_gross_profit, prev_gross_profit.unwrap());
            query::update_earnings_table(
                curr_id,
                (
                    gross_profit_growth_yoy.eq(gp_growth),
                    growth_calculated.eq(true),
                ),
                conn,
            )?;
        }
        Ok(())
    }
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = nominal_earnings)]
pub struct NewNominalEarnings {
    exchange: String,
    ticker: String,
    duration: String,
    quarter_str: i16,
    year_str: i16,
    revenue: f64,
    revenue_growth_yoy: f64,
    cost_of_revenue: f64,
    gross_profit: f64,
    gross_margin: f64,
    gross_profit_growth_yoy: Option<f64>,
    sga_expenses: Option<f64>,
    sga_gp_ratio: Option<f64>,
    rnd_expenses: Option<f64>,
    rnd_gp_ratio: Option<f64>,
    operating_expenses: f64,
    operating_income: f64,
    operating_margin: f64,
    interest_expenses: Option<f64>,
    interest_expenses_op_income_ratio: Option<f64>,
    net_income: f64,
    net_margin: f64,
    eps_basic: f64,
    eps_diluted: f64,
    shares_outstanding_basic: f64,
    shares_outstanding_diluted: f64,
    shares_change_yoy: f64,
    cash_and_equivalents: f64,
    cash_and_short_term_investments: f64,
    accounts_receivable: Option<f64>,
    inventory: Option<f64>,
    total_current_assets: f64,
    goodwill: Option<f64>,
    total_assets: f64,
    accounts_payable: Option<f64>,
    total_current_liabilities: f64,
    total_liabilities: f64,
    retained_earnings: f64,
    shareholders_equity: f64,
    total_debt: f64,
    net_cash: f64,
    depreciation_and_amortization: Option<f64>,
    stock_based_compensation: Option<f64>,
    operating_cash_flow: f64,
    operating_cash_flow_margin: Option<f64>,
    capital_expenditure: Option<f64>,
    investing_cash_flow: f64,
    financing_cash_flow: f64,
    free_cash_flow: f64,
    free_cash_flow_margin: f64,
    ratio_calculated: bool,
    growth_calculated: bool,
}

impl NewNominalEarnings {
    /// adds new earnings data
    pub fn create_new_entry(concat_statement: ConcatStatement) -> Option<Self> {
        if let Some((fiscal_y, fiscal_q)) = process_fiscal_string(&concat_statement.fiscal_quarter)
        {
            Some(NewNominalEarnings {
                exchange: concat_statement.exchange,
                ticker: concat_statement.ticker,
                duration: concat_statement.term,
                quarter_str: fiscal_q,
                year_str: fiscal_y,
                revenue: concat_statement.revenue,
                revenue_growth_yoy: concat_statement.revenue_growth_yoy,
                cost_of_revenue: concat_statement.cost_of_revenue,
                gross_profit: concat_statement.gross_profit,
                gross_margin: concat_statement.gross_margin,
                gross_profit_growth_yoy: None,
                sga_expenses: Some(concat_statement.sga_expenses),
                sga_gp_ratio: None,
                rnd_expenses: Some(concat_statement.rnd_expenses),
                rnd_gp_ratio: None,
                operating_expenses: concat_statement.operating_expenses,
                operating_income: concat_statement.operating_income,
                operating_margin: concat_statement.operating_margin,
                interest_expenses: Some(concat_statement.interest_expenses),
                interest_expenses_op_income_ratio: None,
                net_income: concat_statement.net_income,
                net_margin: concat_statement.net_margin,
                eps_basic: concat_statement.eps_basic,
                eps_diluted: concat_statement.eps_diluted,
                shares_outstanding_basic: concat_statement.shares_outstanding_basic,
                shares_outstanding_diluted: concat_statement.shares_outstanding_diluted,
                shares_change_yoy: concat_statement.shares_change_yoy,
                cash_and_equivalents: concat_statement.cash_and_equivalents,
                cash_and_short_term_investments: concat_statement.cash_and_short_term_investments,
                accounts_receivable: Some(concat_statement.accounts_receivable),
                inventory: Some(concat_statement.inventory),
                total_current_assets: concat_statement.total_current_assets,
                goodwill: Some(concat_statement.goodwill),
                total_assets: concat_statement.total_assets,
                accounts_payable: Some(concat_statement.accounts_payable),
                total_current_liabilities: concat_statement.total_current_liabilities,
                total_liabilities: concat_statement.total_liabilities,
                retained_earnings: concat_statement.retained_earnings,
                shareholders_equity: concat_statement.shareholders_equity,
                total_debt: concat_statement.total_debt,
                net_cash: concat_statement.net_cash,
                depreciation_and_amortization: Some(concat_statement.depreciation_and_amortization),
                stock_based_compensation: Some(concat_statement.stock_based_compensation),
                operating_cash_flow: concat_statement.operating_cash_flow,
                operating_cash_flow_margin: None,
                capital_expenditure: Some(concat_statement.capital_expenditure),
                investing_cash_flow: concat_statement.investing_cash_flow,
                financing_cash_flow: concat_statement.financing_cash_flow,
                free_cash_flow: concat_statement.free_cash_flow,
                free_cash_flow_margin: concat_statement.free_cash_flow_margin,
                ratio_calculated: false,
                growth_calculated: false,
            })
        } else {
            None
        }
    }
}

/// converts fiscal string ("Qn 20xx") into year and quarter integer
fn process_fiscal_string(fiscal_str: &str) -> Option<(i16, i16)> {
    let fiscal_vec: Vec<&str> = fiscal_str.split(' ').collect();
    if fiscal_vec.len() == 1 {
        return None;
    }
    let fiscal_q = match fiscal_vec[0][1..].parse() {
        Ok(q) => q,
        Err(_) => 0, // annual data will be displayed as 0
    };
    let fiscal_y = match fiscal_vec[1].parse() {
        Ok(y) => y,
        Err(_) => 0,
    };
    Some((fiscal_y, fiscal_q))
}

/// inserts multiple earnings to the database
pub fn insert_stock_data_batch(
    stock_entries: Vec<NewNominalEarnings>,
    conn: &mut PgConnection,
) -> Result<bool, DieselError> {
    use crate::schema::nominal_earnings::dsl::*;
    let update_count = diesel::insert_into(nominal_earnings)
        .values(&stock_entries)
        .on_conflict((exchange, ticker, duration, quarter_str, year_str))
        .do_nothing()
        .execute(conn)?;
    Ok(update_count > 0)
}
