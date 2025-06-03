import { Metrics } from "../api/Metrics";
import { HeartIcon } from "@heroicons/react/24/outline";
import EarningsWidget from "./EarningsWidget";

interface MetricsPageProps {
  metrics: Metrics;
  loading: boolean;
  error: string | undefined;
}

function colorcodePriceTarget(
  pt: number | undefined,
  price: number | undefined
) {
  if (pt && price && pt > price) {
    return "text-5xl font-bold p-6 text-green-400";
  } else if (pt && price && pt < price) {
    return "text-5xl font-bold p-6 text-red-400";
  } else {
    return "text-5xl font-bold p-6 text-gray";
  }
}

function getLowestPriceTargetTest(
  targets: (number | undefined)[]
): number | undefined {
  const vals = targets.filter((val) => val !== undefined && val !== null);
  return vals.length > 0 ? Math.min(...vals) : undefined;
}
function TickerMetricsPage(props: MetricsPageProps) {
  const { metrics, loading, error } = props;

  function getLowestPriceTarget(
    targets: (number | undefined)[]
  ): string | undefined {
    const vals = targets.filter((val) => val !== undefined && val !== null);
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
                {metrics?.companyName}
                {" ("}
                {metrics?.ticker.toLocaleUpperCase()}
                {")"}
              </h1>
            </div>
          </div>
          <div className="grid grid-cols-7 p-4">
            <div className="col-span-7 lg:col-span-4 px-2">
              <h2 className="font-bold">Basic Information</h2>
              <div className="p-2 space-y-2.5 flex flex-col">
                <p>
                  <span className="font-bold">Industry: </span>
                  {metrics?.industry}
                </p>
                <p>
                  <span className="font-bold">Next Earnings Date: </span>

                  {metrics?.nextEarningsDate?.toString() ?? "-"}
                </p>
                <EarningsWidget metrics={metrics} />
              </div>
            </div>
            <div className="col-span-7 lg:col-span-3 px-2">
              <h2 className="font-bold">Current Stock Price vs Price Target</h2>
              <div className="flex items-center p-5 justify-center">
                <h1 className="text-5xl font-bold p-6">
                  {metrics?.exchange === "US" ? "$" : "¥"}
                  {metrics?.latestPrice?.toFixed(2) ?? "-"}
                </h1>
                <h3>vs</h3>
                <h1
                  className={colorcodePriceTarget(
                    getLowestPriceTargetTest([
                      metrics?.priceCurrentRevenueGrowth,
                      metrics?.priceMultiYearRevenueGrowth,
                      metrics?.priceMultiYearGpGrowth,
                      metrics?.priceCurrentGpGrowth,
                      metrics?.priceNextYearRevenueGrowth,
                    ]),
                    metrics?.latestPrice
                  )}
                >
                  {metrics?.exchange === "US" ? "$" : "¥"}
                  {getLowestPriceTarget([
                    metrics?.priceCurrentRevenueGrowth,
                    metrics?.priceMultiYearRevenueGrowth,
                    metrics?.priceMultiYearGpGrowth,
                    metrics?.priceCurrentGpGrowth,
                    metrics?.priceNextYearRevenueGrowth,
                  ])}
                </h1>
              </div>
              <h2 className="font-bold">Price target list</h2>
              <div className="p-2 space-y-2.5 flex flex-col">
                <p>
                  Using current revenue growth (
                  {metrics?.revenueGrowthYoyTtm?.toFixed(2) ?? "-"}%):{" "}
                  <span className="text-2xl">
                    {metrics?.exchange === "US" ? "$" : "¥"}
                    {metrics?.priceCurrentRevenueGrowth?.toFixed(2) ?? "-"}
                  </span>
                </p>
                <p>
                  Using multi-year revenue growth (
                  {metrics?.revenueGrowthMultiYear?.toFixed(2) ?? "-"}%):{" "}
                  <span className="text-2xl">
                    {metrics?.exchange === "US" ? "$" : "¥"}
                    {metrics?.priceMultiYearRevenueGrowth?.toFixed(2) ??
                      "-"}{" "}
                  </span>
                </p>
                {metrics?.priceCurrentGpGrowth && (
                  <p>
                    Using current gross profit growth (
                    {metrics?.grossProfitGrowthYoyTtm?.toFixed(2) ?? "-"}
                    %):{" "}
                    <span className="text-2xl">
                      {metrics?.exchange === "US" ? "$" : "¥"}
                      {metrics?.priceCurrentGpGrowth?.toFixed(2) ?? "-"}{" "}
                    </span>
                  </p>
                )}
                {metrics?.priceMultiYearGpGrowth && (
                  <p>
                    Using multi-year gross profit growth (
                    {metrics?.grossProfitGrowthMultiYear?.toFixed(2) ?? "-"}
                    %):{" "}
                    <span className="text-2xl">
                      {metrics?.exchange === "US" ? "$" : "¥"}
                      {metrics?.priceMultiYearGpGrowth?.toFixed(2) ?? "-"}{" "}
                    </span>
                  </p>
                )}
                {metrics?.priceNextYearRevenueGrowth && (
                  <p>
                    Using next year revenue growth forcast (
                    {metrics?.revenueGrowthNextYear?.toFixed(2) ?? "-"}%):{" "}
                    <span className="text-2xl">
                      {metrics?.exchange === "US" ? "$" : "¥"}
                      {metrics?.priceNextYearRevenueGrowth?.toFixed(2) ??
                        "-"}{" "}
                    </span>
                  </p>
                )}
                {/* <p>
                  Simulation (
                  {metrics?.revenueGrowthNextYear?.toFixed(2) ?? "-"}%):{" "}
                  <span className="text-2xl">
                    {metrics?.exchange === "US" ? "$" : "¥"}
                    {metrics?.priceNextYearRevenueGrowth?.toFixed(2) ??
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
                {metrics?.grossMarginLongTermTrend == "Uptrend" &&
                  metrics?.grossMarginShortTermTrend == "Uptrend" && (
                    <li>Gross margin is improving.</li>
                  )}
                {metrics?.sgaLongTermTrend == "Downtrend" && (
                  <li>Customer aquisition effort is shrinking.</li>
                )}
                {metrics?.rndLongTermTrend == "Downtrend" && (
                  <li>R&D cost required is not high.</li>
                )}
                {metrics?.operatingMarginLongTermTrend == "Uptrend" && (
                  <li>The company has higher operating leverge.</li>
                )}
                {typeof metrics?.interestExpenseRatioTtm === "number" &&
                  metrics?.interestExpenseRatioTtm >= -0.15 && (
                    <li>Intrest expense is low.</li>
                  )}
                {typeof metrics?.sharesChangeTtm === "number" &&
                  metrics?.sharesChangeTtm <= 0 && (
                    <li>Share is not diluted.</li>
                  )}
              </ul>
            </div>
            <div className="col-span-2 lg:col-span-1 px-2">
              <h2 className="font-bold">Reasons to avoid</h2>
              <ul className="space-y-2.5">
                {(metrics?.grossMarginLongTermTrend == "Downtrend" ||
                  metrics?.grossMarginShortTermTrend == "Downtrend") && (
                  <li>Gross margin is not improving.</li>
                )}
                {metrics?.sgaLongTermTrend == "Uptrend" && (
                  <li>Customer aquisition effort is expanding.</li>
                )}
                {metrics?.rndLongTermTrend == "Uptrend" && (
                  <li>The company is expanding R&D to survive.</li>
                )}
                {metrics?.operatingMarginLongTermTrend == "Downtrend" && (
                  <li>The company has lower operating leverge.</li>
                )}
                {typeof metrics?.interestExpenseRatioTtm === "number" &&
                  metrics?.interestExpenseRatioTtm < -0.15 && (
                    <li>Intrest expense is high.</li>
                  )}
                {typeof metrics?.sharesChangeTtm === "number" &&
                  metrics?.sharesChangeTtm >= 3 && (
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
