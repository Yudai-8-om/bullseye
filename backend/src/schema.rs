// @generated automatically by Diesel CLI.

diesel::table! {
    companies (id) {
        id -> Int4,
        #[max_length = 50]
        company_name -> Varchar,
        #[max_length = 50]
        industry -> Varchar,
        #[max_length = 12]
        isin -> Varchar,
        #[max_length = 9]
        exchange -> Varchar,
        #[max_length = 6]
        ticker -> Varchar,
        last_updated -> Date,
    }
}

diesel::table! {
    current_metrics (id) {
        id -> Int4,
        company_id -> Int4,
        #[max_length = 3]
        currency -> Varchar,
        net_interest_income_growth_yoy_ttm -> Nullable<Float8>,
        net_interest_income_growth_multi_year -> Nullable<Float8>,
        net_interest_margin_ttm -> Nullable<Float8>,
        net_interest_margin_short_term_trend -> Nullable<Text>,
        net_interest_margin_long_term_trend -> Nullable<Text>,
        cost_of_risk_ttm -> Nullable<Float8>,
        cost_of_risk_short_term_trend -> Nullable<Text>,
        cost_of_risk_long_term_trend -> Nullable<Text>,
        revenue_ttm -> Nullable<Float8>,
        revenue_growth_yoy_ttm -> Nullable<Float8>,
        revenue_growth_multi_year -> Nullable<Float8>,
        gross_profit_growth_yoy_ttm -> Nullable<Float8>,
        gross_profit_growth_multi_year -> Nullable<Float8>,
        gross_margin_ttm -> Nullable<Float8>,
        gross_margin_short_term_trend -> Nullable<Text>,
        gross_margin_long_term_trend -> Nullable<Text>,
        sga_ratio_ttm -> Nullable<Float8>,
        sga_short_term_trend -> Nullable<Text>,
        sga_long_term_trend -> Nullable<Text>,
        rnd_ratio_ttm -> Nullable<Float8>,
        rnd_short_term_trend -> Nullable<Text>,
        rnd_long_term_trend -> Nullable<Text>,
        operating_margin_ttm -> Nullable<Float8>,
        operating_margin_short_term_trend -> Nullable<Text>,
        operating_margin_long_term_trend -> Nullable<Text>,
        interest_expense_ratio_ttm -> Nullable<Float8>,
        net_margin_ttm -> Nullable<Float8>,
        theoretical_net_margin -> Nullable<Float8>,
        is_net_margin_optimized -> Nullable<Bool>,
        shares_outstanding_diluted_ttm -> Nullable<Float8>,
        shares_change_ttm -> Nullable<Float8>,
        shares_change_multi_year -> Nullable<Float8>,
        shares_change_trend -> Nullable<Text>,
        retained_earnings_ttm -> Nullable<Float8>,
        retained_earnings_trend -> Nullable<Text>,
        net_cash_ttm -> Nullable<Float8>,
        has_healthy_net_cash -> Nullable<Bool>,
        net_cash_trend -> Nullable<Text>,
        operating_cash_flow_ttm -> Nullable<Float8>,
        operating_cash_flow_margin_ttm -> Nullable<Float8>,
        operating_cash_flow_margin_trend -> Nullable<Text>,
        free_cash_flow_ttm -> Nullable<Float8>,
        free_cash_flow_margin_ttm -> Nullable<Float8>,
        ffo_margin_ttm -> Nullable<Float8>,
        ffo_margin_trend -> Nullable<Text>,
    }
}

diesel::table! {
    earnings_report (id) {
        id -> Int4,
        company_id -> Int4,
        #[max_length = 1]
        duration -> Varchar,
        quarter_str -> Int2,
        year_str -> Int2,
        period_ending -> Date,
        #[max_length = 3]
        currency -> Varchar,
        net_interest_income -> Nullable<Float8>,
        net_interest_growth_yoy -> Nullable<Float8>,
        net_interest_margin -> Nullable<Float8>,
        provision_for_loan_loss -> Nullable<Float8>,
        cost_of_risk -> Nullable<Float8>,
        revenue -> Float8,
        revenue_growth_yoy -> Nullable<Float8>,
        cost_of_revenue -> Nullable<Float8>,
        gross_profit -> Nullable<Float8>,
        gross_margin -> Nullable<Float8>,
        gross_profit_growth_yoy -> Nullable<Float8>,
        sga_expenses -> Nullable<Float8>,
        sga_gp_ratio -> Nullable<Float8>,
        rnd_expenses -> Nullable<Float8>,
        rnd_gp_ratio -> Nullable<Float8>,
        operating_expenses -> Float8,
        operating_income -> Float8,
        operating_margin -> Float8,
        interest_expenses -> Nullable<Float8>,
        interest_expenses_op_income_ratio -> Nullable<Float8>,
        goodwill_impairment -> Float8,
        net_income -> Float8,
        net_margin -> Float8,
        eps_basic -> Float8,
        eps_diluted -> Float8,
        shares_outstanding_basic -> Float8,
        shares_outstanding_diluted -> Float8,
        shares_change_yoy -> Float8,
        ffo -> Nullable<Float8>,
        ffo_margin -> Nullable<Float8>,
        cash_and_equivalents -> Float8,
        cash_and_short_term_investments -> Nullable<Float8>,
        total_investments -> Nullable<Float8>,
        gross_loans -> Nullable<Float8>,
        accounts_receivable -> Nullable<Float8>,
        inventory -> Nullable<Float8>,
        total_current_assets -> Nullable<Float8>,
        goodwill -> Nullable<Float8>,
        total_assets -> Float8,
        accounts_payable -> Nullable<Float8>,
        total_current_liabilities -> Nullable<Float8>,
        total_liabilities -> Float8,
        retained_earnings -> Float8,
        shareholders_equity -> Float8,
        total_debt -> Nullable<Float8>,
        net_cash -> Float8,
        depreciation_and_amortization -> Nullable<Float8>,
        stock_based_compensation -> Nullable<Float8>,
        operating_cash_flow -> Nullable<Float8>,
        operating_cash_flow_margin -> Nullable<Float8>,
        capital_expenditure -> Nullable<Float8>,
        investing_cash_flow -> Nullable<Float8>,
        financing_cash_flow -> Nullable<Float8>,
        free_cash_flow -> Nullable<Float8>,
        free_cash_flow_margin -> Nullable<Float8>,
        ratio_calculated -> Bool,
        growth_calculated -> Bool,
    }
}

diesel::table! {
    forecasts (id) {
        id -> Int4,
        company_id -> Int4,
        next_earnings_date -> Nullable<Date>,
        latest_price -> Nullable<Float8>,
        last_updated -> Nullable<Date>,
        revenue_next_year -> Nullable<Float8>,
        revenue_growth_next_year -> Nullable<Float8>,
        price_current_revenue_growth -> Nullable<Float8>,
        price_current_gp_growth -> Nullable<Float8>,
        price_next_year_revenue_growth -> Nullable<Float8>,
        price_multi_year_revenue_growth -> Nullable<Float8>,
        price_multi_year_gp_growth -> Nullable<Float8>,
    }
}

diesel::joinable!(current_metrics -> companies (company_id));
diesel::joinable!(earnings_report -> companies (company_id));
diesel::joinable!(forecasts -> companies (company_id));

diesel::allow_tables_to_appear_in_same_query!(
    companies,
    current_metrics,
    earnings_report,
    forecasts,
);
