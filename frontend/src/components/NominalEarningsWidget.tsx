import { NominalMetrics } from "../api/nominalMetrics";

function colorcodeTrend(trend: string | undefined) {
  switch (trend) {
    case "Uptrend":
      return "text-sm text-green-600 font-semibold bg-green-500/20 rounded-full px-0.5";
    case "Downtrend":
      return "text-sm text-red-600 font-semibold bg-red-500/20 rounded-full px-0.5";
    default:
      return "text-sm text-gray-600 font-semibold bg-gray-500/20 rounded-full px-0.5";
  }
}
function colorcodeTrendRev(trend: string | undefined) {
  switch (trend) {
    case "Downtrend":
      return "text-sm text-green-600 font-semibold bg-green-500/20 rounded-full px-0.5";
    case "Uptrend":
      return "text-sm text-red-600 font-semibold bg-red-500/20 rounded-full px-0.5";
    default:
      return "text-sm text-gray-600 font-semibold bg-gray-500/20 rounded-full px-0.5";
  }
}

function interestRatioRedFlag(interestRatio: number | undefined) {
  if (interestRatio && interestRatio < -0.15) {
    return "text-2xl font-bold text-red-600";
  } else {
    return "text-2xl font-bold";
  }
}

function shareDilutionoRedFlag(shareChange: number | undefined) {
  if (shareChange && shareChange > 3) {
    return "text-2xl font-bold text-red-600";
  } else {
    return "text-2xl font-bold";
  }
}

function RevenueWidget({ nominalMetrics }: { nominalMetrics: NominalMetrics }) {
  return (
    <div className="grid grid-cols-6 gap-3">
      <div className="flex flex-col col-span-full  bg-white rounded-xl sm:col-span-3 xl:col-span-2">
        <div className="pl-4 pr-1 pt-1.5 ">
          <header className="flex font-semibold text-sm">Revenue (TTM)</header>
          <div className="flex items-center">
            <div className="text-2xl font-bold">
              {nominalMetrics?.currency}
              {nominalMetrics?.revenueTtm ?? "-"}
            </div>
            <div className="px-3 flex flex-col w-full">
              <div className="flex items-center justify-between mb-1">
                <div className="text-xs">YoY</div>
                <div className="text-sm text-green-600 font-semibold bg-green-500/20 rounded-full px-0.5">
                  {nominalMetrics?.revenueGrowthYoyTtm?.toFixed(2) ?? "-"}%
                </div>
              </div>
              <div className="flex items-center justify-between">
                <div className="text-xs">4yr</div>
                <div className="text-sm text-green-600 font-semibold bg-green-500/20 rounded-full px-0.5">
                  {nominalMetrics?.revenueGrowthMultiYear?.toFixed(2) ?? "-"}%
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
      <div className="flex flex-col col-span-full  bg-white rounded-xl sm:col-span-3 xl:col-span-2">
        <div className="pl-4 pr-1 pt-1.5">
          <header className="flex font-semibold text-sm">
            Gross Margin (TTM)
          </header>
          <div className="flex items-center">
            <div className="text-2xl font-bold">
              {nominalMetrics?.grossMarginTtm?.toFixed(2) ?? "-"}%
            </div>
            <div className="px-4 flex flex-col w-full">
              <div className="flex items-center justify-between mb-1">
                <div className="text-xs">S-trend</div>
                <div
                  className={colorcodeTrend(
                    nominalMetrics?.grossMarginShortTermTrend
                  )}
                >
                  {nominalMetrics?.grossMarginShortTermTrend ?? "-"}
                </div>
              </div>
              <div className="flex items-center justify-between">
                <div className="text-xs">L-trend</div>
                <div
                  className={colorcodeTrend(
                    nominalMetrics?.grossMarginLongTermTrend
                  )}
                >
                  {nominalMetrics?.grossMarginLongTermTrend ?? "-"}
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div className="flex flex-col col-span-full  bg-white rounded-xl sm:col-span-3 xl:col-span-2">
        <div className="pl-4 pr-1 pt-1.5">
          <header className="flex font-semibold text-sm">
            Operating Margin (TTM)
          </header>
          <div className="flex items-center">
            <div className="text-2xl font-bold">
              {nominalMetrics?.operatingMarginTtm?.toFixed(2) ?? "-"}%
            </div>
            <div className="px-4 flex flex-col w-full">
              <div className="flex items-center justify-between mb-1">
                <div className="text-xs">S-trend</div>
                <div
                  className={colorcodeTrend(
                    nominalMetrics?.operatingMarginShortTermTrend
                  )}
                >
                  {nominalMetrics?.operatingMarginShortTermTrend ?? "-"}
                </div>
              </div>
              <div className="flex items-center justify-between">
                <div className="text-xs">L-trend</div>
                <div
                  className={colorcodeTrend(
                    nominalMetrics?.operatingMarginLongTermTrend
                  )}
                >
                  {nominalMetrics?.operatingMarginLongTermTrend ?? "-"}
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div className="flex flex-col col-span-full  bg-white rounded-xl sm:col-span-3 xl:col-span-2">
        <div className="pl-4 pr-1 pt-1.5">
          <header className="flex font-semibold text-sm">
            SGA Ratio (TTM)
          </header>
          <div className="flex items-center">
            <div className="text-2xl font-bold">
              {nominalMetrics?.sgaRatioTtm?.toFixed(2) ?? "-"}
            </div>
            <div className="px-4 flex flex-col w-full">
              <div className="flex items-center justify-between mb-1">
                <div className="text-xs">S-trend</div>
                <div
                  className={colorcodeTrendRev(
                    nominalMetrics?.sgaShortTermTrend
                  )}
                >
                  {nominalMetrics?.sgaShortTermTrend ?? "-"}
                </div>
              </div>
              <div className="flex items-center justify-between">
                <div className="text-xs">L-trend</div>
                <div
                  className={colorcodeTrendRev(
                    nominalMetrics?.sgaLongTermTrend
                  )}
                >
                  {nominalMetrics?.sgaLongTermTrend ?? "-"}
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div className="flex flex-col col-span-full  bg-white rounded-xl sm:col-span-3 xl:col-span-2">
        <div className="pl-4 pr-1 pt-1.5">
          <header className="flex font-semibold text-sm">
            R&D Ratio (TTM)
          </header>
          <div className="flex items-center">
            <div className="text-2xl font-bold">
              {nominalMetrics?.rndRatioTtm?.toFixed(2) ?? "-"}
            </div>
            <div className="px-4 flex flex-col w-full">
              <div className="flex items-center justify-between mb-1">
                <div className="text-xs">S-trend</div>
                <div
                  className={colorcodeTrendRev(
                    nominalMetrics?.rndShortTermTrend
                  )}
                >
                  {nominalMetrics?.rndShortTermTrend ?? "-"}
                </div>
              </div>
              <div className="flex items-center justify-between">
                <div className="text-xs">L-trend</div>
                <div
                  className={colorcodeTrendRev(
                    nominalMetrics?.rndLongTermTrend
                  )}
                >
                  {nominalMetrics?.rndLongTermTrend ?? "-"}
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div className="flex flex-col col-span-full  bg-white rounded-xl sm:col-span-3 xl:col-span-2">
        <div className="pl-4 pr-1 pt-1.5">
          <header className="flex font-semibold text-sm">
            OCF Margin (TTM)
          </header>
          <div className="flex items-center">
            <div className="text-2xl font-bold">
              {nominalMetrics?.operatingCashFlowMarginTtm?.toFixed(2) ?? "-"}%
            </div>
            <div className="px-4 flex flex-col w-full">
              <div className="flex items-center justify-between mb-1">
                <div className="text-xs">Trend</div>
                <div
                  className={colorcodeTrend(
                    nominalMetrics?.operatingMarginShortTermTrend
                  )}
                >
                  {nominalMetrics?.operatingMarginShortTermTrend ?? "-"}
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
      <div className="flex flex-col col-span-full  bg-white rounded-xl sm:col-span-3 xl:col-span-2">
        <div className="pl-4 pr-1 pt-1.5">
          <header className="flex font-semibold text-sm">
            Interest Expense Ratio (TTM)
          </header>
          <div className="flex items-center">
            <div
              className={interestRatioRedFlag(
                nominalMetrics?.interestExpenseRatioTtm
              )}
            >
              {nominalMetrics?.interestExpenseRatioTtm?.toFixed(2) ?? "-"}
            </div>
            <div className="px-4 flex flex-col w-full">
              <div className="flex items-center justify-between mb-1">
                <div className="text-xs">Trend</div>
                <div
                  className={colorcodeTrendRev(
                    nominalMetrics?.sharesChangeTrend
                  )}
                >
                  {nominalMetrics?.sharesChangeTrend ?? "-"}
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
      <div className="flex flex-col col-span-full  bg-white rounded-xl sm:col-span-3 xl:col-span-2">
        <div className="pl-4 pr-1 pt-1.5">
          <header className="flex font-semibold text-sm">
            Share Count Change
          </header>
          <div className="flex items-center">
            <div
              className={shareDilutionoRedFlag(nominalMetrics?.sharesChangeTtm)}
            >
              {nominalMetrics?.sharesChangeTtm?.toFixed(2) ?? "-"}%
            </div>
            <div className="px-4 flex flex-col w-full">
              <div className="flex items-center justify-between mb-1">
                <div className="text-xs">Trend</div>
                <div
                  className={colorcodeTrendRev(
                    nominalMetrics?.sharesChangeTrend
                  )}
                >
                  {nominalMetrics?.sharesChangeTrend ?? "-"}
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
      {/* 
      
      <p>
        <span className="font-bold">Net Margin (TTM): </span>
        {nominalMetrics?.netMarginTtm?.toFixed(2) ?? "-"}%
      </p>
      <p>
        <span className="font-bold">Theoretical Net Margin: </span>
        {nominalMetrics?.theoreticalNetMargin?.toFixed(2) ?? "-"}%
      </p>
      <p>
        <span className="font-bold">Operating Cash Flow Margin (TTM): </span>
        {nominalMetrics?.operatingCashFlowMarginTtm?.toFixed(2) ?? "-"}%
      </p>
      <p>
        <span className="font-bold">Free Cash Flow Margin (TTM): </span>
        {nominalMetrics?.freeCashFlowMarginTtm?.toFixed(2) ?? "-"}%
      </p> */}
    </div>
  );
}

export default RevenueWidget;
