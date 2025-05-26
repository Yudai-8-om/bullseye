import { bullseyeAPI } from "../api/bullseyeAPI";
import { StockHealthEval } from "../api/stockHealthEval";
import { useState } from "react";
import SearchBar from "./SearchBar";
import { HeartIcon } from "@heroicons/react/24/outline";

function SingleTickerPage() {
  const [healthEval, updateEvals] = useState<StockHealthEval>();
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | undefined>(undefined);
  //   const [ticker, getTicker] = useState("");

  async function loadEvals(ticker: string) {
    console.log("Got Ticker: ", ticker);
    setLoading(true);
    try {
      const eval_data = await bullseyeAPI.get(ticker);
      console.log(JSON.stringify(eval_data, null, 2));
      updateEvals(eval_data);
      setError(undefined);
    } catch (e) {
      if (e instanceof Error) {
        console.log("Errorrrrrrr!");
        setError(e.message);
      } else {
        console.log("Unexpected Error: ", e);
      }
    } finally {
      setLoading(false);
    }
  }
  function getLowestPriceTarget(
    targets: (number | undefined)[]
  ): string | undefined {
    const vals = targets.filter((val) => val !== undefined);
    return vals.length > 0 ? Math.min(...vals).toFixed(2) : undefined;
  }

  return (
    <>
      <header>
        <SearchBar onSearch={(ticker) => loadEvals(ticker)} />
      </header>

      {error && (
        <div className="bg-red-400">
          <div className="card large error">
            <section>
              <p>
                <span className="icon-alert inverse"></span>
                {error}
              </p>
            </section>
          </div>
        </div>
      )}
      {loading && (
        <div className="center-page">
          <span className="spinner primary"></span>
          <p className="text-2xl">Loading...</p>
        </div>
      )}
      {(!loading || !error) && (
        <>
          <div>
            <div className="flex items-center p-3">
              <HeartIcon className="h-8 w-8 text-red-400"></HeartIcon>
              <h1 className="text-4xl font-bold">
                {healthEval?.ticker.toLocaleUpperCase()}
              </h1>
            </div>
          </div>
          <div className="grid grid-cols-2 p-4">
            <div>
              <h2 className="font-bold">Basic Information</h2>
              <div className="p-2 space-y-2.5 flex flex-col">
                <p>
                  <span className="font-bold">Industry: </span>
                  {healthEval?.industry}
                </p>
                <p>
                  <span className="font-bold">Next Earnings Date: </span>

                  {healthEval?.nextEarningsDate?.toString() ?? "-"}
                </p>
                <p>
                  <span className="font-bold">Revenue (TTM): </span>{" "}
                  {healthEval?.currency}
                  {healthEval?.latestRevenue ?? "-"}
                </p>
                <p>
                  <span className="font-bold">Revenue Growth YoY (TTM): </span>{" "}
                  {healthEval?.revenueGrowth1y?.toFixed(2) ?? "-"}%
                </p>
                <p>
                  <span className="font-bold">
                    Gross Profit Growth YoY (TTM):{" "}
                  </span>{" "}
                  {healthEval?.grossProfitGrowth1y?.toFixed(2) ?? "-"}%
                </p>
                <p>
                  <span className="font-bold">
                    Revenue Growth YoY 4yr Average:{" "}
                  </span>{" "}
                  {healthEval?.revenueGrowthMultiYear?.toFixed(2) ?? "-"}%
                </p>
                <p>
                  <span className="font-bold">Gross Margin (TTM): </span>
                  {healthEval?.latestGrossMargin?.toFixed(2) ?? "-"}%
                </p>
                <p>
                  <span className="font-bold">Operating Margin (TTM): </span>
                  {healthEval?.latestOperatingMargin?.toFixed(2) ?? "-"}%
                </p>
                <p>
                  <span className="font-bold">Net Margin (TTM): </span>
                  {healthEval?.latestNetMargin?.toFixed(2) ?? "-"}%
                </p>
                <p>
                  <span className="font-bold">Theoretical Net Margin: </span>
                  {healthEval?.theoreticalNetMargin?.toFixed(2) ?? "-"}%
                </p>
              </div>
            </div>
            <div>
              <h2 className="font-bold">Current Stock Price</h2>
              <div className="flex items-center p-5">
                <h1 className="text-5xl font-bold p-6">
                  {healthEval?.exchange === "US" ? "$" : "¥"}
                  {healthEval?.latestPrice?.toFixed(2) ?? "-"}
                </h1>
                <h3>vs</h3>
                <h1 className="text-5xl font-bold p-6 text-green-400">
                  {healthEval?.exchange === "US" ? "$" : "¥"}
                  {getLowestPriceTarget([
                    healthEval?.priceCurrentRevenueGrowth,
                    healthEval?.priceMultiYearRevenueGrowth,
                    healthEval?.priceMultiYearGpGrowth,
                    healthEval?.priceCurrentGpGrowth,
                    healthEval?.priceNextYearRevenueGrowth,
                  ])}
                </h1>
              </div>
              <h2 className="font-bold">Price target</h2>
              <div className="p-2 space-y-2.5 flex flex-col">
                <p>
                  Using current revenue growth (
                  {healthEval?.revenueGrowth1y?.toFixed(2) ?? "-"}%):{" "}
                  <span className="text-2xl">
                    {healthEval?.exchange === "US" ? "$" : "¥"}
                    {healthEval?.priceCurrentRevenueGrowth?.toFixed(2) ?? "-"}
                  </span>
                </p>
                <p>
                  Using multi-year revenue growth (
                  {healthEval?.revenueGrowthMultiYear?.toFixed(2) ?? "-"}%):{" "}
                  <span className="text-2xl">
                    {healthEval?.exchange === "US" ? "$" : "¥"}
                    {healthEval?.priceMultiYearRevenueGrowth?.toFixed(2) ??
                      "-"}{" "}
                  </span>
                </p>
                <p>
                  Using current gross profit growth (
                  {healthEval?.grossProfitGrowth1y?.toFixed(2) ?? "-"}%):{" "}
                  <span className="text-2xl">
                    {healthEval?.exchange === "US" ? "$" : "¥"}
                    {healthEval?.priceCurrentGpGrowth?.toFixed(2) ?? "-"}{" "}
                  </span>
                </p>
                <p>
                  Using multi-year gross profit growth (
                  {healthEval?.grossProfitGrowthMultiYear?.toFixed(2) ?? "-"}%):{" "}
                  <span className="text-2xl">
                    {healthEval?.exchange === "US" ? "$" : "¥"}
                    {healthEval?.priceMultiYearGpGrowth?.toFixed(2) ?? "-"}{" "}
                  </span>
                </p>
                <p>
                  Using next year revenue growth forcast (
                  {healthEval?.revenueGrowthNextYear?.toFixed(2) ?? "-"}%):{" "}
                  <span className="text-2xl">
                    {healthEval?.exchange === "US" ? "$" : "¥"}
                    {healthEval?.priceNextYearRevenueGrowth?.toFixed(2) ??
                      "-"}{" "}
                  </span>
                </p>
                <p>
                  Simulation (
                  {healthEval?.revenueGrowthNextYear?.toFixed(2) ?? "-"}%):{" "}
                  <span className="text-2xl">
                    {healthEval?.exchange === "US" ? "$" : "¥"}
                    {healthEval?.priceNextYearRevenueGrowth?.toFixed(2) ??
                      "-"}{" "}
                  </span>
                </p>
              </div>
            </div>
          </div>
          <div className="grid grid-cols-2 p-4">
            <div className="">
              <h2 className="font-bold">Reasons to buy</h2>
              <ul className="space-y-2.5">
                {healthEval?.improvingGrossMargin && (
                  <li>Gross margin is improving.</li>
                )}
                {healthEval?.lowCustomerAcquisition && (
                  <li>Customer aquisition effort is low.</li>
                )}
                {healthEval?.lowInnovation && (
                  <li>R&D cost required is not high.</li>
                )}
                {healthEval?.lowInterestBurden && (
                  <li>Intrest expense is low.</li>
                )}
                {healthEval?.positiveRetainedEarnings && (
                  <li>
                    The company has excess money to use for share buybacks and
                    dividends.
                  </li>
                )}
                {healthEval?.noShareDilution && <li>Share is not diluted.</li>}
              </ul>
            </div>
            <div className="p-4">
              <h2 className="font-bold">Reasons to avoid</h2>
              <ul className="space-y-2.5">
                {!healthEval?.improvingGrossMargin && (
                  <li>Gross margin is not improving.</li>
                )}
                {!healthEval?.lowCustomerAcquisition && (
                  <li>Customer aquisition effort is high.</li>
                )}
                {!healthEval?.lowInnovation && (
                  <li>R&D cost required is high.</li>
                )}
                {!healthEval?.lowInterestBurden && (
                  <li>Intrest expense is high.</li>
                )}
                {!healthEval?.positiveRetainedEarnings && (
                  <li>
                    The company doesn't have a room share buybacks and
                    dividends.
                  </li>
                )}
                {!healthEval?.noShareDilution && (
                  <li>Share is excessively diluted.</li>
                )}
              </ul>
            </div>
          </div>
        </>
      )}
    </>
  );
}
export default SingleTickerPage;
