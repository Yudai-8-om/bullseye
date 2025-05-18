use crate::calculate;
use crate::errors::BullsEyeError;
use crate::metrics;
use crate::query;
use crate::schema::{stock_data, stock_health_eval};
use bullseye_api;
use bullseye_api::table::ConcatStatement;
use chrono::{Duration, Local, NaiveDate};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::Error as DieselError;
use serde::{Deserialize, Serialize};

pub fn establish_connection() -> PgConnection {
    let database_url = "postgres://testuser:stock@localhost/bullseye_db";
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn establish_connection_pool() -> Result<Pool<ConnectionManager<PgConnection>>, BullsEyeError> {
    let database_url = "postgres://testuser:stock@localhost/bullseye_db";
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager);
    match pool {
        Ok(val) => Ok(val),
        Err(_) => Err(BullsEyeError::DbPoolError),
    }
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = stock_data)]
pub struct StockData {
    id: i32,
    exchange: String,
    ticker: String,
    duration: String,
    quarter_str: i16,
    year_str: i16,
    revenue: f64,
    revenue_growth_yoy: f64,
    cost_of_revenue: Option<f64>,
    gross_profit: Option<f64>,
    gross_margin: Option<f64>,
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
    accounts_receivable: f64,
    inventory: f64,
    total_current_assets: f64,
    goodwill: f64,
    total_assets: f64,
    accounts_payable: f64,
    total_current_liabilities: f64,
    total_liabilities: f64,
    retained_earnings: f64,
    shareholders_equity: f64,
    total_debt: f64,
    net_cash: f64,
    depreciation_and_amortization: f64,
    stock_based_compensation: f64,
    operating_cash_flow: f64,
    capital_expenditure: f64,
    investing_cash_flow: f64,
    financing_cash_flow: f64,
    free_cash_flow: f64,
    free_cash_flow_margin: f64,
    ratio_calculated: bool,
    growth_calculated: bool,
}
impl StockData {
    pub fn quarter(&self) -> i16 {
        self.quarter_str
    }
    pub fn revenue(&self) -> f64 {
        self.revenue
    }
    pub fn revenue_growth_yoy(&self) -> f64 {
        self.revenue_growth_yoy
    }
    pub fn gross_profit(&self) -> Option<f64> {
        self.gross_profit
    }
    pub fn gross_profit_growth_yoy(&self) -> Option<f64> {
        self.gross_profit_growth_yoy
    }
    pub fn gross_margin(&self) -> Option<f64> {
        self.gross_margin
    }
    pub fn operating_income(&self) -> f64 {
        self.operating_income
    }
    pub fn operating_margin(&self) -> f64 {
        self.operating_margin
    }
    pub fn net_income(&self) -> f64 {
        self.net_income
    }
    pub fn net_margin(&self) -> f64 {
        self.net_margin
    }
    fn sga_expenses(&self) -> Option<f64> {
        self.sga_expenses
    }
    pub fn sga_gp_ratio(&self) -> Option<f64> {
        self.sga_gp_ratio
    }
    fn rnd_expenses(&self) -> Option<f64> {
        self.rnd_expenses
    }
    pub fn rnd_gp_ratio(&self) -> Option<f64> {
        self.rnd_gp_ratio
    }
    pub fn interest_expenses(&self) -> Option<f64> {
        self.interest_expenses
    }
    pub fn interest_expenses_op_income_ratio(&self) -> Option<f64> {
        self.interest_expenses_op_income_ratio
    }
    pub fn shares_outstanding_diluted(&self) -> f64 {
        self.shares_outstanding_diluted
    }
    pub fn retained_earnings(&self) -> f64 {
        self.retained_earnings
    }
    pub fn shares_change_yoy(&self) -> f64 {
        self.shares_change_yoy
    }
    pub fn net_cash(&self) -> f64 {
        self.net_cash
    }
    pub fn operating_cash_flow(&self) -> f64 {
        self.operating_cash_flow
    }
    pub fn latest_quarter_data(
        target_ticker: &str,
        target_exchange: &str,
        conn: &mut PgConnection,
    ) -> Result<Self, DieselError> {
        use crate::schema::stock_data::dsl::*;
        query::load_first_row(
            stock_data
                .filter(ticker.eq(target_ticker))
                .filter(exchange.eq(target_exchange))
                .filter(duration.eq("T"))
                .order((year_str.desc(), quarter_str.desc())),
            conn,
        )
    }
    pub fn latest_annual_data(
        target_ticker: &str,
        target_exchange: &str,
        conn: &mut PgConnection,
    ) -> Result<Self, DieselError> {
        use crate::schema::stock_data::dsl::*;
        query::load_first_row(
            stock_data
                .filter(ticker.eq(target_ticker))
                .filter(exchange.eq(target_exchange))
                .filter(duration.eq("Y"))
                .order(year_str.desc()),
            conn,
        )
    }
    pub fn prev_year_data(&self, conn: &mut PgConnection) -> Result<Option<Self>, DieselError> {
        let curr_exchange = &self.exchange;
        let curr_duration = &self.duration;
        let curr_ticker = &self.ticker;
        let curr_year = self.year_str;
        let curr_quarter = self.quarter_str;
        use crate::schema::stock_data::dsl::*;
        let prev_year_result = query::load_first_row(
            stock_data
                .filter(exchange.eq(curr_exchange))
                .filter(ticker.eq(curr_ticker))
                .filter(duration.eq(curr_duration))
                .filter(year_str.eq(curr_year - 1))
                .filter(quarter_str.eq(curr_quarter)),
            conn,
        );
        let prev_year = match prev_year_result {
            Ok(data) => Ok(Some(data)),
            Err(DieselError::NotFound) => Ok(None),
            Err(e) => Err(e),
        };
        prev_year
    }

    fn update_ratios(&self, conn: &mut PgConnection) -> Result<(), DieselError> {
        use crate::schema::stock_data::dsl::*;
        let curr_id = self.id;
        let sga_ratio = self
            .gross_profit()
            .map(|gp| calculate::calculate_ratio(self.sga_expenses(), gp))
            .flatten();
        let rnd_ratio = self
            .gross_profit()
            .map(|gp| calculate::calculate_ratio(self.rnd_expenses(), gp))
            .flatten();
        let interest_ratio =
            calculate::calculate_ratio(self.interest_expenses(), self.operating_income());
        diesel::update(stock_data.filter(id.eq(curr_id)))
            .set((
                sga_gp_ratio.eq(sga_ratio),
                rnd_gp_ratio.eq(rnd_ratio),
                interest_expenses_op_income_ratio.eq(interest_ratio),
                ratio_calculated.eq(true),
            ))
            .execute(conn)?;
        Ok(())
    }
    fn update_yoy_gp_growth(&self, conn: &mut PgConnection) -> Result<(), DieselError> {
        use crate::schema::stock_data::dsl::*;
        let curr_id = self.id;
        let curr_gross_profit = match self.gross_profit() {
            Some(0.) | None => {
                diesel::update(stock_data.filter(id.eq(curr_id)))
                    .set(growth_calculated.eq(true))
                    .execute(conn)?;
                return Ok(());
            }
            Some(val) => val,
        };
        let prev_year_data = self.prev_year_data(conn)?;
        let prev_gross_profit_result = prev_year_data.map(|data| data.gross_profit()).flatten();
        let prev_gross_profit = match prev_gross_profit_result {
            Some(0.) | None => {
                diesel::update(stock_data.filter(id.eq(curr_id)))
                    .set(growth_calculated.eq(true))
                    .execute(conn)?;
                return Ok(());
            }
            Some(val) => val,
        };
        let gp_growth = calculate::calculate_yoy_growth(curr_gross_profit, prev_gross_profit);
        diesel::update(stock_data.filter(id.eq(curr_id)))
            .set((
                gross_profit_growth_yoy.eq(gp_growth),
                growth_calculated.eq(true),
            ))
            .execute(conn)?;
        Ok(())
    }
}

pub fn extract_field<T, F>(data: &Vec<StockData>, f: F) -> Vec<T>
where
    F: Fn(&StockData) -> T,
{
    data.iter().map(f).collect()
}

pub fn update_ratios_batch(conn: &mut PgConnection) -> Result<(), DieselError> {
    use crate::schema::stock_data::dsl::*;
    let target: Vec<StockData> = stock_data
        .filter(ratio_calculated.eq(false))
        .load::<StockData>(conn)?;
    for i in target {
        i.update_ratios(conn)?;
    }
    Ok(())
}

pub fn update_growths(conn: &mut PgConnection) -> Result<(), DieselError> {
    use crate::schema::stock_data::dsl::*;
    let target = stock_data
        .filter(growth_calculated.eq(false))
        .filter(gross_profit.ne(0.))
        .load::<StockData>(conn)?;
    for i in target {
        i.update_yoy_gp_growth(conn)?;
    }
    Ok(())
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = stock_data)]
pub struct NewStockEntry {
    exchange: String,
    ticker: String,
    duration: String,
    quarter_str: i16,
    year_str: i16,
    revenue: f64,
    revenue_growth_yoy: f64,
    cost_of_revenue: Option<f64>,
    gross_profit: Option<f64>,
    gross_margin: Option<f64>,
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
    accounts_receivable: f64,
    inventory: f64,
    total_current_assets: f64,
    goodwill: f64,
    total_assets: f64,
    accounts_payable: f64,
    total_current_liabilities: f64,
    total_liabilities: f64,
    retained_earnings: f64,
    shareholders_equity: f64,
    total_debt: f64,
    net_cash: f64,
    depreciation_and_amortization: f64,
    stock_based_compensation: f64,
    operating_cash_flow: f64,
    capital_expenditure: f64,
    investing_cash_flow: f64,
    financing_cash_flow: f64,
    free_cash_flow: f64,
    free_cash_flow_margin: f64,
    ratio_calculated: bool,
    growth_calculated: bool,
}

impl NewStockEntry {
    pub fn add_new_entry(concat_statement: ConcatStatement) -> Option<Self> {
        let fiscal: Vec<&str> = concat_statement.fiscal_quarter.split(' ').collect();
        if fiscal.len() == 1 {
            return None;
        }
        let fiscal_q = match fiscal[0][1..].parse() {
            Ok(q) => q,
            Err(_) => 0, // annual data will be displayed as 0
        };
        let fiscal_y = match fiscal[1].parse() {
            Ok(y) => y,
            Err(_) => 0,
        };
        Some(NewStockEntry {
            exchange: concat_statement.exchange,
            ticker: concat_statement.ticker,
            duration: concat_statement.term,
            quarter_str: fiscal_q,
            year_str: fiscal_y,
            revenue: concat_statement.revenue,
            revenue_growth_yoy: concat_statement.revenue_growth_yoy,
            cost_of_revenue: Some(concat_statement.cost_of_revenue),
            gross_profit: Some(concat_statement.gross_profit),
            gross_margin: Some(concat_statement.gross_margin),
            gross_profit_growth_yoy: Some(concat_statement.gross_profit_growth_yoy),
            sga_expenses: Some(concat_statement.sga_expenses),
            sga_gp_ratio: Some(0.),
            rnd_expenses: Some(concat_statement.rnd_expenses),
            rnd_gp_ratio: Some(0.),
            operating_expenses: concat_statement.operating_expenses,
            operating_income: concat_statement.operating_income,
            operating_margin: concat_statement.operating_margin,
            interest_expenses: Some(concat_statement.interest_expenses),
            interest_expenses_op_income_ratio: Some(0.),
            net_income: concat_statement.net_income,
            net_margin: concat_statement.net_margin,
            eps_basic: concat_statement.eps_basic,
            eps_diluted: concat_statement.eps_diluted,
            shares_outstanding_basic: concat_statement.shares_outstanding_basic,
            shares_outstanding_diluted: concat_statement.shares_outstanding_diluted,
            shares_change_yoy: concat_statement.shares_change_yoy,
            cash_and_equivalents: concat_statement.cash_and_equivalents,
            cash_and_short_term_investments: concat_statement.cash_and_short_term_investments,
            accounts_receivable: concat_statement.accounts_receivable,
            inventory: concat_statement.inventory,
            total_current_assets: concat_statement.total_current_assets,
            goodwill: concat_statement.goodwill,
            total_assets: concat_statement.total_assets,
            accounts_payable: concat_statement.accounts_payable,
            total_current_liabilities: concat_statement.total_current_liabilities,
            total_liabilities: concat_statement.total_liabilities,
            retained_earnings: concat_statement.retained_earnings,
            shareholders_equity: concat_statement.shareholders_equity,
            total_debt: concat_statement.total_debt,
            net_cash: concat_statement.net_cash,
            depreciation_and_amortization: concat_statement.depreciation_and_amortization,
            stock_based_compensation: concat_statement.stock_based_compensation,
            operating_cash_flow: concat_statement.operating_cash_flow,
            capital_expenditure: concat_statement.capital_expenditure,
            investing_cash_flow: concat_statement.investing_cash_flow,
            financing_cash_flow: concat_statement.financing_cash_flow,
            free_cash_flow: concat_statement.free_cash_flow,
            free_cash_flow_margin: concat_statement.free_cash_flow_margin,
            ratio_calculated: false,
            growth_calculated: false,
        })
    }
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = stock_health_eval)]
#[serde(rename_all = "camelCase")]
pub struct StockHealthEval {
    id: i32,
    exchange: String,
    ticker: String,
    currency: String,
    favorite: bool,
    industry: String,
    next_earnings_date: Option<NaiveDate>,
    latest_price: Option<f64>,
    positive_operating_cash_flow: Option<bool>,
    operating_cash_flow_health: Option<bool>, // 20+% or improving
    improving_gross_margin: Option<bool>,     // Moving average
    low_customer_acquisition: Option<bool>,
    improving_customer_acquisition: Option<bool>,
    low_innovation: Option<bool>,
    improving_innovation: Option<bool>,
    low_interest_burden: Option<bool>,
    healthy_net_cash: Option<bool>,
    positive_retained_earnings: Option<bool>,
    improving_retained_earnings: Option<bool>, //YOY or QoQ comparison
    no_share_dilution: Option<bool>,
    improving_share_dilution: Option<bool>,
    latest_revenue: Option<f64>,
    revenue_next_year: Option<f64>,
    revenue_growth_1y: Option<f64>,
    revenue_growth_multi_year: Option<f64>,
    revenue_growth_next_year: Option<f64>,
    gross_profit_growth_1y: Option<f64>,
    gross_profit_growth_multi_year: Option<f64>,
    latest_gross_margin: Option<f64>,
    latest_operating_margin: Option<f64>,
    latest_net_margin: Option<f64>,
    latest_eps: Option<f64>,
    latest_operating_cash_flow: Option<f64>,
    latest_free_cash_flow: Option<f64>,
    theoretical_net_margin: Option<f64>,
    theoretical_net_income: Option<f64>,
    theoretical_net_income_next_year: Option<f64>,
    optimized_net_margin: Option<bool>, // by comparing with theoretical
    price_current_revenue_growth: Option<f64>,
    price_current_gp_growth: Option<f64>,
    price_next_year_revenue_growth: Option<f64>,
    price_multi_year_revenue_growth: Option<f64>,
    price_multi_year_gp_growth: Option<f64>,
    last_updated: Option<NaiveDate>,
}
impl StockHealthEval {
    fn industry(&self) -> &str {
        &self.industry
    }
    pub fn next_earnings_date(&self) -> Option<NaiveDate> {
        self.next_earnings_date
    }
    fn revenue_next_year(&self) -> Option<f64> {
        self.revenue_next_year
    }
    fn revenue_growth_multi_year(&self) -> Option<f64> {
        self.revenue_growth_multi_year
    }
    fn gross_profit_growth_multi_year(&self) -> Option<f64> {
        self.gross_profit_growth_multi_year
    }
    fn latest_net_margin(&self) -> Option<f64> {
        self.latest_net_margin
    }
    fn theoretical_net_margin(&self) -> Option<f64> {
        self.theoretical_net_margin
    }
    fn optimized_net_margin(&self) -> Option<bool> {
        self.optimized_net_margin
    }
    pub fn last_updated(&self) -> Option<NaiveDate> {
        self.last_updated
    }
    pub fn search(target_ticker: &str, exc: &str, conn: &mut PgConnection) -> Self {
        use crate::schema::stock_health_eval::dsl::*;
        let target = stock_health_eval
            .filter(ticker.eq(target_ticker))
            .filter(exchange.eq(exc))
            .first::<StockHealthEval>(conn)
            .expect("Cannot load database. Failed to update StockHealth table");
        target
    }
    pub fn is_ticker_existed(symbol: &str, exc: &str, conn: &mut PgConnection) -> bool {
        use crate::schema::stock_health_eval::dsl::*;
        let ticker_exists = diesel::dsl::select(diesel::dsl::exists(
            stock_health_eval
                .filter(ticker.eq(symbol))
                .filter(exchange.eq(exc)),
        ))
        .get_result::<bool>(conn);
        match ticker_exists {
            Ok(exist_bool) => exist_bool,
            Err(_) => false,
        }
    }

    fn assess_basic_health(
        &self,
        target_ticker: &str,
        target_exchange: &str,
        conn: &mut PgConnection,
    ) -> Result<(), DieselError> {
        use crate::schema::stock_data;
        use crate::schema::stock_data::dsl::*;
        use crate::schema::stock_health_eval::dsl::*;
        let target: StockData = query::load_first_row(
            stock_data
                .filter(stock_data::ticker.eq(target_ticker))
                .filter(stock_data::exchange.eq(target_exchange))
                .filter(duration.eq("T"))
                .filter(gross_margin.ne(0.))
                .order((year_str.desc(), quarter_str.desc())),
            conn,
        )?; //TODO: Fix here
        let curr_revenue = target.revenue();
        let curr_revenue_growth_yoy = target.revenue_growth_yoy();
        let curr_gross_profit_growth_yoy = target.gross_profit_growth_yoy();
        let curr_gross_margin = target.gross_margin();
        let curr_operating_margin = target.operating_margin();
        // let curr_net_income = target.net_income();
        let curr_net_margin = target.net_margin();
        let curr_shares_change_yoy = target.shares_change_yoy();
        let is_ocf_positive = metrics::is_ocf_positive(&target);
        let is_rnd_light = metrics::is_rnd_light(&target);
        let is_sga_light = metrics::is_sga_light(&target);
        let has_low_interest_expense = metrics::has_low_interest_expense(&target);
        let is_active_share_buyback = metrics::is_active_share_buyback(&target);
        let is_share_diluted = metrics::is_share_diluted(&target);
        let has_healthy_cash_position = metrics::has_healthy_cash_position(&target);
        let is_room_for_buyback = metrics::is_room_for_buyback(&target);
        let industry_name = self.industry();
        let net_margin_factor = get_net_margin_factor(industry_name);

        let (curr_theoretical_net_margin, is_net_margin_optimized) =
            metrics::is_net_margin_optimized(&target, net_margin_factor);

        let curr_theoretical_net_income = match is_net_margin_optimized {
            true => curr_revenue * curr_net_margin / 100.,
            false => curr_revenue * curr_theoretical_net_margin / 100.,
        };
        let curr_theoretical_eps =
            curr_theoretical_net_income / target.shares_outstanding_diluted();

        let curr_theoretical_price_rev = calculate::calculate_price_target(
            curr_theoretical_eps,
            curr_revenue_growth_yoy,
            curr_shares_change_yoy,
        );
        let curr_theoretical_price_gp = curr_gross_profit_growth_yoy.map(|val| {
            calculate::calculate_price_target(curr_theoretical_eps, val, curr_shares_change_yoy)
        });
        let curr_theoretical_price_multi_rev = self.revenue_growth_multi_year().map(|val| {
            calculate::calculate_price_target(curr_theoretical_eps, val, curr_shares_change_yoy)
        });
        let curr_theoretical_price_multi_gp = self.gross_profit_growth_multi_year().map(|val| {
            calculate::calculate_price_target(curr_theoretical_eps, val, curr_shares_change_yoy)
        });

        query::update_eval_table(
            target_ticker,
            target_exchange,
            (
                positive_operating_cash_flow.eq(is_ocf_positive),
                low_customer_acquisition.eq(is_sga_light),
                low_innovation.eq(is_rnd_light),
                low_interest_burden.eq(has_low_interest_expense),
                healthy_net_cash.eq(has_healthy_cash_position),
                positive_retained_earnings.eq(is_room_for_buyback),
                no_share_dilution.eq(is_active_share_buyback),
                latest_revenue.eq(curr_revenue),
                revenue_growth_1y.eq(curr_revenue_growth_yoy),
                gross_profit_growth_1y.eq(curr_gross_profit_growth_yoy),
                latest_gross_margin.eq(curr_gross_margin),
                latest_operating_margin.eq(curr_operating_margin),
                latest_net_margin.eq(curr_net_margin),
                theoretical_net_margin.eq(curr_theoretical_net_margin),
                theoretical_net_income.eq(curr_theoretical_net_income),
                optimized_net_margin.eq(is_net_margin_optimized),
                price_current_revenue_growth.eq(curr_theoretical_price_rev),
                price_current_gp_growth.eq(curr_theoretical_price_gp),
                price_multi_year_revenue_growth.eq(curr_theoretical_price_multi_rev),
                price_multi_year_gp_growth.eq(curr_theoretical_price_multi_gp),
            ),
            conn,
        )?;
        Ok(())
    }

    fn assess_estimate(
        &self,
        target_ticker: &str,
        target_exchange: &str,
        conn: &mut PgConnection,
    ) -> Result<(), DieselError> {
        use crate::schema::stock_health_eval::dsl::*;
        let target = StockData::latest_annual_data(target_ticker, target_exchange, conn)?;
        let curr_rev = target.revenue();
        let next_yr_rev = self.revenue_next_year();
        let next_yr_rev_growth =
            next_yr_rev.map(|val| calculate::calculate_yoy_growth(val, curr_rev));
        let curr_net_margin_optimized = self.optimized_net_margin();
        let curr_theoretical_net_margin = self.theoretical_net_margin();
        let curr_net_margin = self.latest_net_margin();
        let curr_shares_diluted = target.shares_outstanding_diluted();
        let curr_shares_change_yoy = target.shares_change_yoy();
        let next_theoretical_net_income = match curr_net_margin_optimized {
            Some(true) => {
                next_yr_rev.and_then(|rev: f64| curr_net_margin.map(|margin| rev * margin / 100.))
            }
            Some(false) => next_yr_rev
                .and_then(|rev| curr_theoretical_net_margin.map(|margin| rev * margin / 100.)),
            None => None,
        };
        let next_theoretical_eps =
            next_theoretical_net_income.map(|income| income / curr_shares_diluted);
        let next_yr_theoretical_price = next_theoretical_eps.and_then(|eps| {
            next_yr_rev_growth.map(|growth| {
                eps * calculate::calculate_growth_adjustment_factor(growth - curr_shares_change_yoy)
            })
        });
        query::update_eval_table(
            target_ticker,
            target_exchange,
            (
                revenue_growth_next_year.eq(next_yr_rev_growth),
                theoretical_net_income_next_year.eq(next_theoretical_net_income),
                price_next_year_revenue_growth.eq(next_yr_theoretical_price),
            ),
            conn,
        )?;
        Ok(())
    }

    fn update_trend(
        &self,
        target_ticker: &str,
        target_exchange: &str,
        conn: &mut PgConnection,
    ) -> Result<(), DieselError> {
        use crate::schema::stock_health_eval::dsl::*;
        let target = query::load_multiple_earnings_ttm(target_ticker, target_exchange, 8, conn)?;
        let (is_uptrend, _) = metrics::get_gross_margin_trend_short(target, 8);
        query::update_eval_table(
            target_ticker,
            target_exchange,
            (improving_gross_margin.eq(is_uptrend),),
            conn,
        )?;
        Ok(())
    }

    fn calculate_multi_yr_rev_growth(
        &self,
        target_ticker: &str,
        target_exchange: &str,
        conn: &mut PgConnection,
    ) -> Result<(), DieselError> {
        use crate::schema::stock_health_eval::dsl::*;
        let target = query::load_multiple_earnings_annual(target_ticker, target_exchange, 4, conn)?;
        let rev_growth = extract_field(&target, |data| data.revenue_growth_yoy());
        let rev_growth_ave = calculate::calculate_average_growth(rev_growth);
        query::update_eval_table(
            target_ticker,
            target_exchange,
            revenue_growth_multi_year.eq(rev_growth_ave),
            conn,
        )?;
        Ok(())
    }

    fn calculate_multi_yr_gp_growth(
        &self,
        target_ticker: &str,
        target_exchange: &str,
        conn: &mut PgConnection,
    ) -> Result<(), DieselError> {
        use crate::schema::stock_data::dsl::*;
        use crate::schema::stock_health_eval::dsl::*;
        let target = query::load_multiple_earnings_annual_filter(
            target_ticker,
            target_exchange,
            |q| q.filter(gross_profit_growth_yoy.ne(0.)),
            4,
            conn,
        )?;
        let gp_growth: Vec<f64> = target
            .iter()
            .filter_map(|i| i.gross_profit_growth_yoy())
            .collect();
        let gp_growth_ave = calculate::calculate_average_growth(gp_growth);
        query::update_eval_table(
            target_ticker,
            target_exchange,
            gross_profit_growth_multi_year.eq(gp_growth_ave),
            conn,
        )?;
        Ok(())
    }
}

pub fn update_earnings_date(
    target_ticker: &str,
    target_exchange: &str,
    earnings_date: Option<String>,
    conn: &mut PgConnection,
) -> Result<(), DieselError> {
    use crate::schema::stock_health_eval::dsl::*;
    let next_earnings =
        earnings_date.map(|date_str| NaiveDate::parse_from_str(&date_str, "%b %d, %Y").unwrap());
    let valid_next_earnings = next_earnings.filter(|&date| {
        date >= Local::now().date_naive() || Local::now().date_naive() - date <= Duration::days(3)
    });
    query::update_eval_table(
        target_ticker,
        target_exchange,
        next_earnings_date.eq(valid_next_earnings),
        conn,
    )?;
    Ok(())
}
pub fn empty_earnings_date(
    target_ticker: &str,
    target_exchange: &str,
    conn: &mut PgConnection,
) -> Result<(), DieselError> {
    use crate::schema::stock_health_eval::dsl::*;
    query::update_eval_table(
        target_ticker,
        target_exchange,
        next_earnings_date.eq::<Option<NaiveDate>>(None),
        conn,
    )?;
    Ok(())
}

pub fn update_price(
    target_ticker: &str,
    target_exchange: &str,
    price: Option<f64>,
    conn: &mut PgConnection,
) -> Result<(), DieselError> {
    use crate::schema::stock_health_eval::dsl::*;
    query::update_eval_table(
        target_ticker,
        target_exchange,
        (
            latest_price.eq(price),
            last_updated.eq(Local::now().date_naive()),
        ),
        conn,
    )?;
    Ok(())
}
pub fn update_estimate(
    target_ticker: &str,
    target_exchange: &str,
    next_yr_rev: Option<f64>,
    conn: &mut PgConnection,
) -> Result<(), DieselError> {
    use crate::schema::stock_health_eval::dsl::*;
    query::update_eval_table(
        target_ticker,
        target_exchange,
        revenue_next_year.eq(next_yr_rev),
        conn,
    )?;
    Ok(())
}

pub fn run_eval_prep<'a>(
    symbol: &str,
    exc: &str,
    conn: &mut PgConnection,
) -> Result<(), DieselError> {
    use crate::schema::stock_health_eval::dsl::*;
    let target: StockHealthEval = stock_health_eval
        .filter(ticker.eq(symbol))
        .filter(exchange.eq(exc))
        .first::<StockHealthEval>(conn)
        .expect("cannot load database");
    target.calculate_multi_yr_rev_growth(symbol, exc, conn)?;
    target.calculate_multi_yr_gp_growth(symbol, exc, conn)?;
    target.update_trend(symbol, exc, conn)?;
    Ok(())
}

pub fn run_eval<'a>(symbol: &str, exc: &str, conn: &mut PgConnection) -> Result<(), DieselError> {
    let target = StockHealthEval::search(symbol, exc, conn);
    target.assess_basic_health(symbol, exc, conn)?;
    let reloaded_target = StockHealthEval::search(symbol, exc, conn);
    reloaded_target.assess_estimate(symbol, exc, conn)?;
    Ok(())
}

pub fn run_sim<'a>(
    symbol: &str,
    exc: &str,
    sim_net_margin: u8,
    sim_growth: u8,
    conn: &mut PgConnection,
) -> f64 {
    use crate::schema::stock_data::dsl::*;
    let target = stock_data
        .filter(ticker.eq(symbol))
        .filter(exchange.eq(exc))
        .filter(duration.eq("T"))
        .order((year_str.desc(), quarter_str.desc()))
        .limit(1)
        .first::<StockData>(conn)
        .expect("Cannot load database. Failed to simulate");
    let sim_net_margin: f64 = sim_net_margin as f64;
    let sim_growth: f64 = sim_growth as f64;
    let sim_theoretical_eps =
        target.revenue() * sim_net_margin / 100. / target.shares_outstanding_diluted();
    let sim_price = sim_theoretical_eps
        * calculate::calculate_growth_adjustment_factor(sim_growth - target.shares_change_yoy());
    sim_price
}

fn get_net_margin_factor(industry: &str) -> f64 {
    match industry {
        "Airlines" => 20.,
        "Grocery Stores" | "Department Stores" => 9.,
        "Discount Stores" => 6.,
        "Apparel Retail" | "Apparel Manufacturing" | "Footwear & Accessories" => 5.,
        "Banks - Diversified" | "Banks - Regional" => 4.,
        "Internet Retail" | "Specialty Retail" => 3.5,
        "Specialty Industrial Machinery" => 2.5,
        "Semiconductors"
        | "Semiconductor Equipment & Materials"
        | "Auto Manufacturers"
        | "Auto Parts"
        | "Consumer Electronics"
        | "Electrical Equipment & Parts" => 2.,
        _ => 3.,
    }
}

#[derive(Insertable)]
#[diesel(table_name = stock_health_eval)]
struct NewStockHealthEval<'a> {
    exchange: &'a str,
    ticker: &'a str,
    currency: &'a str,
    favorite: bool,
    industry: String,
    next_earnings_date: Option<NaiveDate>,
    latest_price: Option<f64>,
    positive_operating_cash_flow: Option<bool>,
    operating_cash_flow_health: Option<bool>,
    improving_gross_margin: Option<bool>,
    low_customer_acquisition: Option<bool>,
    improving_customer_acquisition: Option<bool>,
    low_innovation: Option<bool>,
    improving_innovation: Option<bool>,
    low_interest_burden: Option<bool>,
    healthy_net_cash: Option<bool>,
    positive_retained_earnings: Option<bool>,
    improving_retained_earnings: Option<bool>,
    no_share_dilution: Option<bool>,
    improving_share_dilution: Option<bool>,
    latest_revenue: Option<f64>,
    revenue_next_year: Option<f64>,
    revenue_growth_1y: Option<f64>,
    revenue_growth_multi_year: Option<f64>,
    revenue_growth_next_year: Option<f64>,
    gross_profit_growth_1y: Option<f64>,
    gross_profit_growth_multi_year: Option<f64>,
    latest_gross_margin: Option<f64>,
    latest_operating_margin: Option<f64>,
    latest_net_margin: Option<f64>,
    latest_eps: Option<f64>,
    latest_operating_cash_flow: Option<f64>,
    latest_free_cash_flow: Option<f64>,
    theoretical_net_margin: Option<f64>,
    theoretical_net_income: Option<f64>,
    theoretical_net_income_next_year: Option<f64>,
    optimized_net_margin: Option<bool>,
    price_current_revenue_growth: Option<f64>,
    price_current_gp_growth: Option<f64>,
    price_next_year_revenue_growth: Option<f64>,
    price_multi_year_revenue_growth: Option<f64>,
    price_multi_year_gp_growth: Option<f64>,
    last_updated: Option<NaiveDate>,
}

impl<'a> NewStockHealthEval<'a> {
    async fn create_new_entry(
        exc: &bullseye_api::table::Exchange,
        symbol: &'a str,
        currency_str: &'a str,
        industry_str: &'a str,
        next_earnings: Option<NaiveDate>,
        price: Option<f64>,
        next_yr_rev: Option<f64>,
    ) -> Self {
        NewStockHealthEval {
            exchange: bullseye_api::table::get_exchange_string(exc),
            ticker: symbol,
            currency: currency_str,
            favorite: false,
            industry: industry_str.to_string(),
            next_earnings_date: next_earnings,
            latest_price: price,
            positive_operating_cash_flow: None,
            operating_cash_flow_health: None,
            improving_gross_margin: None,
            low_customer_acquisition: None,
            improving_customer_acquisition: None,
            low_innovation: None,
            improving_innovation: None,
            low_interest_burden: None,
            healthy_net_cash: None,
            positive_retained_earnings: None,
            improving_retained_earnings: None,
            no_share_dilution: None,
            improving_share_dilution: None,
            latest_revenue: None,
            revenue_next_year: next_yr_rev,
            revenue_growth_1y: None,
            revenue_growth_multi_year: None,
            revenue_growth_next_year: None,
            gross_profit_growth_1y: None,
            gross_profit_growth_multi_year: None,
            latest_gross_margin: None,
            latest_operating_margin: None,
            latest_net_margin: None,
            latest_eps: None,
            latest_operating_cash_flow: None,
            latest_free_cash_flow: None,
            theoretical_net_margin: None,
            theoretical_net_income: None,
            theoretical_net_income_next_year: None,
            optimized_net_margin: None,
            price_current_revenue_growth: None,
            price_current_gp_growth: None,
            price_next_year_revenue_growth: None,
            price_multi_year_revenue_growth: None,
            price_multi_year_gp_growth: None,
            last_updated: Some(Local::now().date_naive()),
        }
    }
}

pub async fn add_new_eval<'a>(
    symbol: &str,
    exc: &bullseye_api::table::Exchange,
    currency_str: &str,
    industry_str: &str,
    earnings_date: Option<String>,
    price: Option<f64>,
    next_yr_rev: Option<f64>,
    conn: &mut PgConnection,
) {
    use crate::schema::stock_health_eval::dsl::*;
    let next_earnings = earnings_date
        .map(|date_str: String| NaiveDate::parse_from_str(&date_str, "%b %d, %Y").unwrap());
    let valid_next_earnings = next_earnings.filter(|&date| {
        date >= Local::now().date_naive() || Local::now().date_naive() - date <= Duration::days(3)
    });
    let new_entry: NewStockHealthEval<'_> = NewStockHealthEval::create_new_entry(
        exc,
        symbol,
        currency_str,
        industry_str,
        valid_next_earnings,
        price,
        next_yr_rev,
    )
    .await;

    diesel::insert_into(stock_health_eval)
        .values(&new_entry)
        .on_conflict((exchange, ticker))
        .do_nothing()
        .execute(conn)
        .expect("Failed to insert new entry into stock health eval table");
}

pub fn insert_stock_data_batch(stock_entries: Vec<NewStockEntry>, conn: &mut PgConnection) {
    use crate::schema::stock_data::dsl::*;
    diesel::insert_into(stock_data)
        .values(&stock_entries)
        .on_conflict((exchange, ticker, duration, quarter_str, year_str))
        .do_nothing()
        .execute(conn)
        .expect("Failed to insert new entry into stock data table");
}

// pub fn load_stock_data(conn: &mut PgConnection) -> Vec<StockData> {
//     use crate::schema::stock_data::dsl::*;
//     let result = stock_data
//         .limit(10)
//         // .select((ticker))
//         .select(StockData::as_select())
//         .load(conn)
//         .expect("Error loading data");
//     result
// }

// pub fn load_health_data(conn: &mut PgConnection) -> Vec<StockHealthEval> {
//     use crate::schema::stock_health_eval::dsl::*;
//     let result = stock_health_eval
//         .limit(10)
//         // .select((ticker))
//         .select(StockHealthEval::as_select())
//         .load(conn)
//         .expect("Error loading data");
//     result
// }
