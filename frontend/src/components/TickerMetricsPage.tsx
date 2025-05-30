import { NominalMetrics } from "../api/nominalMetrics";
import { HeartIcon } from "@heroicons/react/24/outline";
import RevenueWidget from "./NominalEarningsWidget";

interface MetricsPageProps {
  nominalMetrics: NominalMetrics;
  loading: boolean;
  error: string | undefined;
}

function TickerMetricsPage(props: MetricsPageProps) {
  const { nominalMetrics, loading, error } = props;

  function getLowestPriceTarget(
    targets: (number | undefined)[]
  ): string | undefined {
    const vals = targets.filter((val) => val !== undefined);
    return vals.length > 0 ? Math.min(...vals).toFixed(2) : undefined;
  }
  return (
    <div>
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
      {!loading && !error && (
        <div className="px-4 sm:px-5 lg:px-6 py-6 w-full max-w-9xl mx-auto">
          <div className="rounded-xl bg-white gap">
            <div className="flex items-center p-4">
              <HeartIcon className="h-8 w-8 text-red-400 m-1"></HeartIcon>
              <h1 className="text-4xl font-bold">
                {nominalMetrics?.ticker.toLocaleUpperCase()}
              </h1>
            </div>
          </div>
          <div className="grid grid-cols-2 p-4">
            <div className="col-span-2 lg:col-span-1 px-2">
              <h2 className="font-bold">Basic Information</h2>
              <div className="p-2 space-y-2.5 flex flex-col">
                <p>
                  <span className="font-bold">Industry: </span>
                  {nominalMetrics?.industry}
                </p>
                <p>
                  <span className="font-bold">Next Earnings Date: </span>

                  {nominalMetrics?.nextEarningsDate?.toString() ?? "-"}
                </p>
                <RevenueWidget nominalMetrics={nominalMetrics} />

                <p>
                  <span className="font-bold">Net Margin (TTM): </span>
                  {nominalMetrics?.netMarginTtm?.toFixed(2) ?? "-"}%
                </p>
                <p>
                  <span className="font-bold">Theoretical Net Margin: </span>
                  {nominalMetrics?.theoreticalNetMargin?.toFixed(2) ?? "-"}%
                </p>
                <p>
                  <span className="font-bold">
                    Free Cash Flow Margin (TTM):{" "}
                  </span>
                  {nominalMetrics?.freeCashFlowMarginTtm?.toFixed(2) ?? "-"}%
                </p>
              </div>
            </div>
            <div className="col-span-2 lg:col-span-1 px-2">
              <h2 className="font-bold">Current Stock Price</h2>
              <div className="flex items-center p-5">
                <h1 className="text-5xl font-bold p-6">
                  {nominalMetrics?.exchange === "US" ? "$" : "¥"}
                  {nominalMetrics?.latestPrice?.toFixed(2) ?? "-"}
                </h1>
                <h3>vs</h3>
                <h1 className="text-5xl font-bold p-6 text-green-400">
                  {nominalMetrics?.exchange === "US" ? "$" : "¥"}
                  {getLowestPriceTarget([
                    nominalMetrics?.priceCurrentRevenueGrowth,
                    nominalMetrics?.priceMultiYearRevenueGrowth,
                    nominalMetrics?.priceMultiYearGpGrowth,
                    nominalMetrics?.priceCurrentGpGrowth,
                    nominalMetrics?.priceNextYearRevenueGrowth,
                  ])}
                </h1>
              </div>
              <h2 className="font-bold">Price target</h2>
              <div className="p-2 space-y-2.5 flex flex-col">
                <p>
                  Using current revenue growth (
                  {nominalMetrics?.revenueGrowthYoyTtm?.toFixed(2) ?? "-"}%):{" "}
                  <span className="text-2xl">
                    {nominalMetrics?.exchange === "US" ? "$" : "¥"}
                    {nominalMetrics?.priceCurrentRevenueGrowth?.toFixed(2) ??
                      "-"}
                  </span>
                </p>
                <p>
                  Using multi-year revenue growth (
                  {nominalMetrics?.revenueGrowthMultiYear?.toFixed(2) ?? "-"}%):{" "}
                  <span className="text-2xl">
                    {nominalMetrics?.exchange === "US" ? "$" : "¥"}
                    {nominalMetrics?.priceMultiYearRevenueGrowth?.toFixed(2) ??
                      "-"}{" "}
                  </span>
                </p>
                <p>
                  Using current gross profit growth (
                  {nominalMetrics?.grossProfitGrowthYoyTtm?.toFixed(2) ?? "-"}
                  %):{" "}
                  <span className="text-2xl">
                    {nominalMetrics?.exchange === "US" ? "$" : "¥"}
                    {nominalMetrics?.priceCurrentGpGrowth?.toFixed(2) ??
                      "-"}{" "}
                  </span>
                </p>
                <p>
                  Using multi-year gross profit growth (
                  {nominalMetrics?.grossProfitGrowthMultiYear?.toFixed(2) ??
                    "-"}
                  %):{" "}
                  <span className="text-2xl">
                    {nominalMetrics?.exchange === "US" ? "$" : "¥"}
                    {nominalMetrics?.priceMultiYearGpGrowth?.toFixed(2) ??
                      "-"}{" "}
                  </span>
                </p>
                <p>
                  Using next year revenue growth forcast (
                  {nominalMetrics?.revenueGrowthNextYear?.toFixed(2) ?? "-"}%):{" "}
                  <span className="text-2xl">
                    {nominalMetrics?.exchange === "US" ? "$" : "¥"}
                    {nominalMetrics?.priceNextYearRevenueGrowth?.toFixed(2) ??
                      "-"}{" "}
                  </span>
                </p>
                {/* <p>
                  Simulation (
                  {nominalMetrics?.revenueGrowthNextYear?.toFixed(2) ?? "-"}%):{" "}
                  <span className="text-2xl">
                    {nominalMetrics?.exchange === "US" ? "$" : "¥"}
                    {nominalMetrics?.priceNextYearRevenueGrowth?.toFixed(2) ??
                      "-"}{" "}
                  </span>
                </p> */}
              </div>
            </div>
          </div>
          <div className="grid grid-cols-2 p-4">
            <div className="col-span-2 lg:col-span-1 px-2">
              <h2 className="font-bold">Reasons to buy</h2>
              <ul className="space-y-2.5">
                {nominalMetrics?.grossMarginLongTermTrend == "Uptrend" &&
                  nominalMetrics?.grossMarginShortTermTrend == "Uptrend" && (
                    <li>Gross margin is improving.</li>
                  )}
                {nominalMetrics?.sgaLongTermTrend == "Downtrend" && (
                  <li>Customer aquisition effort is shrinking.</li>
                )}
                {nominalMetrics?.rndLongTermTrend == "Downtrend" && (
                  <li>R&D cost required is not high.</li>
                )}
                {nominalMetrics?.operatingMarginLongTermTrend == "Uptrend" && (
                  <li>The company has higher operating leverge.</li>
                )}
                {typeof nominalMetrics?.interestExpenseRatioTtm === "number" &&
                  nominalMetrics?.interestExpenseRatioTtm >= -0.15 && (
                    <li>Intrest expense is low.</li>
                  )}
                {typeof nominalMetrics?.sharesChangeTtm === "number" &&
                  nominalMetrics?.sharesChangeTtm <= 0 && (
                    <li>Share is not diluted.</li>
                  )}
              </ul>
            </div>
            <div className="col-span-2 lg:col-span-1 px-2">
              <h2 className="font-bold">Reasons to avoid</h2>
              <ul className="space-y-2.5">
                {(nominalMetrics?.grossMarginLongTermTrend == "Downtrend" ||
                  nominalMetrics?.grossMarginShortTermTrend == "Downtrend") && (
                  <li>Gross margin is not improving.</li>
                )}
                {nominalMetrics?.sgaLongTermTrend == "Uptrend" && (
                  <li>Customer aquisition effort is expanding.</li>
                )}
                {nominalMetrics?.rndLongTermTrend == "Uptrend" && (
                  <li>The company is expanding R&D to survive.</li>
                )}
                {nominalMetrics?.operatingMarginLongTermTrend ==
                  "Downtrend" && (
                  <li>The company has lower operating leverge.</li>
                )}
                {typeof nominalMetrics?.interestExpenseRatioTtm === "number" &&
                  nominalMetrics?.interestExpenseRatioTtm < -0.15 && (
                    <li>Intrest expense is high.</li>
                  )}
                {typeof nominalMetrics?.sharesChangeTtm === "number" &&
                  nominalMetrics?.sharesChangeTtm >= 3 && (
                    <li>Share is excessively diluted.</li>
                  )}
              </ul>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
export default TickerMetricsPage;
