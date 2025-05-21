-- This file should undo anything in `up.sql`
DROP INDEX same_ticker_on_nominal_metrics;
DROP INDEX same_nominal_earnings;
DROP TABLE nominal_metrics;
DROP TABLE nominal_earnings;