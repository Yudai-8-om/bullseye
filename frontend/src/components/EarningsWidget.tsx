import { Metrics } from "../api/Metrics";
import SimpleCard from "./SimpleCard";
import MetricsCardWithSingleIndicator from "./SingleIndicatorCard";
import MetricsCardWithTwoIndicators from "./TwoIndicatorsCard";
import { getCurrencySymbol } from "../utils/currency";

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

// function interestRatioRedFlag(interestRatio: number | undefined) {
//   if (interestRatio && interestRatio < -0.15) {
//     return "text-2xl font-bold text-red-600";
//   } else {
//     return "text-2xl font-bold";
//   }
// }

// function shareDilutionoRedFlag(shareChange: number | undefined) {
//   if (shareChange && shareChange > 3) {
//     return "text-2xl font-bold text-red-600";
//   } else {
//     return "text-2xl font-bold";
//   }
// }

function EarningsWidget({ metrics }: { metrics: Metrics }) {
  return (
    <div className="grid grid-cols-6 gap-3">
      <div className="flex flex-col col-span-full  bg-white rounded-xl sm:col-span-3 xl:col-span-2">
        <MetricsCardWithTwoIndicators
          title="Revenue (TTM) in M"
          value={`${getCurrencySymbol(metrics?.currency)}${
            metrics?.revenueTtm ?? "-"
          }`}
          firstIndicatorTitle="YoY"
          firstIndicator={`${metrics?.revenueGrowthYoyTtm?.toFixed(2) ?? "-"}%`}
          firstClassName="text-sm text-green-600 font-semibold bg-green-500/20 rounded-full px-0.5"
          secondIndicatorTitle="4yr"
          secondIndicator={` ${
            metrics?.revenueGrowthMultiYear?.toFixed(2) ?? "-"
          }%`}
          secondClassName="text-sm text-green-600 font-semibold bg-green-500/20 rounded-full px-0.5"
        />
      </div>

      {metrics?.netInterestMarginTtm && (
        <div className="flex flex-col col-span-full  bg-white rounded-xl sm:col-span-3 xl:col-span-2">
          <MetricsCardWithTwoIndicators
            title="NIM (TTM)"
            value={`${metrics?.netInterestMarginTtm?.toFixed(2) ?? "-"}%`}
            firstIndicatorTitle="Short-term"
            firstIndicator={`${
              metrics?.netInterestMarginShortTermTrend ?? "-"
            }`}
            firstClassName={colorcodeTrend(
              metrics?.netInterestMarginShortTermTrend
            )}
            secondIndicatorTitle="Long-term"
            secondIndicator={` ${
              metrics?.netInterestMarginLongTermTrend ?? "-"
            }`}
            secondClassName={colorcodeTrend(
              metrics?.netInterestMarginLongTermTrend
            )}
          />
        </div>
      )}

      {metrics?.costOfRiskTtm && (
        <div className="flex flex-col col-span-full  bg-white rounded-xl sm:col-span-3 xl:col-span-2">
          <MetricsCardWithTwoIndicators
            title="Cost of Risk (TTM)"
            value={`${metrics?.costOfRiskTtm?.toFixed(2) ?? "-"}%`}
            firstIndicatorTitle="Short-term"
            firstIndicator={`${metrics?.costOfRiskShortTermTrend ?? "-"}`}
            firstClassName={colorcodeTrendRev(
              metrics?.costOfRiskShortTermTrend
            )}
            secondIndicatorTitle="Long-term"
            secondIndicator={` ${metrics?.costOfRiskLongTermTrend ?? "-"}`}
            secondClassName={colorcodeTrendRev(
              metrics?.costOfRiskLongTermTrend
            )}
          />
        </div>
      )}

      {metrics?.grossMarginTtm && (
        <div className="flex flex-col col-span-full  bg-white rounded-xl sm:col-span-3 xl:col-span-2">
          <MetricsCardWithTwoIndicators
            title="Gross Margin (TTM)"
            value={`${metrics?.grossMarginTtm?.toFixed(2) ?? "-"}%`}
            firstIndicatorTitle="Short-term"
            firstIndicator={`${metrics?.grossMarginShortTermTrend ?? "-"}`}
            firstClassName={colorcodeTrend(metrics?.grossMarginShortTermTrend)}
            secondIndicatorTitle="Long-term"
            secondIndicator={` ${metrics?.grossMarginLongTermTrend ?? "-"}`}
            secondClassName={colorcodeTrend(metrics?.grossMarginLongTermTrend)}
          />
        </div>
      )}

      <div className="flex flex-col col-span-full  bg-white rounded-xl sm:col-span-3 xl:col-span-2">
        <MetricsCardWithTwoIndicators
          title="Operating Margin (TTM)"
          value={`${metrics?.operatingMarginTtm?.toFixed(2) ?? "-"}%`}
          firstIndicatorTitle="Short-term"
          firstIndicator={`${metrics?.operatingMarginShortTermTrend ?? "-"}`}
          firstClassName={colorcodeTrend(
            metrics?.operatingMarginShortTermTrend
          )}
          secondIndicatorTitle="Long-term"
          secondIndicator={` ${metrics?.operatingMarginLongTermTrend ?? "-"}`}
          secondClassName={colorcodeTrend(
            metrics?.operatingMarginLongTermTrend
          )}
        />
      </div>

      {metrics?.sgaRatioTtm && (
        <div className="flex flex-col col-span-full  bg-white rounded-xl sm:col-span-3 xl:col-span-2">
          <MetricsCardWithTwoIndicators
            title="SGA Ratio (TTM)"
            value={`${metrics?.sgaRatioTtm?.toFixed(2) ?? "-"}`}
            firstIndicatorTitle="Short-term"
            firstIndicator={`${metrics?.sgaShortTermTrend ?? "-"}`}
            firstClassName={colorcodeTrendRev(metrics?.sgaShortTermTrend)}
            secondIndicatorTitle="Long-term"
            secondIndicator={` ${metrics?.sgaLongTermTrend ?? "-"}`}
            secondClassName={colorcodeTrendRev(metrics?.sgaLongTermTrend)}
          />
        </div>
      )}

      {(metrics?.rndRatioTtm === 0 || metrics?.rndRatioTtm) && (
        <div className="flex flex-col col-span-full  bg-white rounded-xl sm:col-span-3 xl:col-span-2">
          <MetricsCardWithTwoIndicators
            title="R&D Ratio (TTM)"
            value={`${metrics?.rndRatioTtm?.toFixed(2) ?? "-"}`}
            firstIndicatorTitle="Short-term"
            firstIndicator={`${metrics?.rndShortTermTrend ?? "-"}`}
            firstClassName={colorcodeTrendRev(metrics?.rndShortTermTrend)}
            secondIndicatorTitle="Long-term"
            secondIndicator={` ${metrics?.rndLongTermTrend ?? "-"}`}
            secondClassName={colorcodeTrendRev(metrics?.rndLongTermTrend)}
          />
        </div>
      )}
      <div className="flex flex-col col-span-full  bg-white rounded-xl sm:col-span-3 xl:col-span-2">
        <MetricsCardWithSingleIndicator
          title="Net Margin (TTM)"
          value={`${metrics?.netMarginTtm?.toFixed(2) ?? "-"}%`}
          indicatorTitle="Theoretical"
          indicator={
            metrics?.theoreticalNetMargin
              ? `${metrics?.theoreticalNetMargin?.toFixed(2) ?? "-"}%`
              : "-"
          }
          className="text-sm text-black font-semibold px-0.5"
        />
      </div>
      {(metrics?.ffoMarginTtm === 0 || metrics?.ffoMarginTtm) && (
        <div className="flex flex-col col-span-full  bg-white rounded-xl sm:col-span-3 xl:col-span-2">
          <MetricsCardWithSingleIndicator
            title="FFO Margin (TTM)"
            value={`${metrics?.ffoMarginTtm?.toFixed(2) ?? "-"}%`}
            indicatorTitle="Long-term"
            indicator={metrics?.ffoMarginTrend ?? "-"}
            className={colorcodeTrend(metrics?.operatingCashFlowMarginTrend)}
          />
        </div>
      )}
      {(metrics?.operatingCashFlowMarginTtm === 0 ||
        metrics?.operatingCashFlowMarginTtm) && (
        <div className="flex flex-col col-span-full  bg-white rounded-xl sm:col-span-3 xl:col-span-2">
          <MetricsCardWithSingleIndicator
            title="OCF Margin (TTM)"
            value={`${metrics?.operatingCashFlowMarginTtm?.toFixed(2) ?? "-"}%`}
            indicatorTitle="Long-term"
            indicator={metrics?.operatingCashFlowMarginTrend ?? "-"}
            className={colorcodeTrend(metrics?.operatingCashFlowMarginTrend)}
          />
        </div>
      )}
      {(metrics?.freeCashFlowMarginTtm === 0 ||
        metrics?.freeCashFlowMarginTtm) && (
        <div className="flex flex-col col-span-full  bg-white rounded-xl sm:col-span-3 xl:col-span-2">
          <SimpleCard
            title="FCF Margin (TTM)"
            value={`${metrics?.freeCashFlowMarginTtm?.toFixed(2) ?? "-"}%`}
          />
        </div>
      )}

      {(metrics?.interestExpenseRatioTtm === 0 ||
        metrics?.interestExpenseRatioTtm) && (
        <div className="flex flex-col col-span-full  bg-white rounded-xl sm:col-span-3 xl:col-span-2">
          <SimpleCard
            title="Interest Expense Ratio (TTM)"
            value={`${metrics?.interestExpenseRatioTtm?.toFixed(2) ?? "-"}`}
          />
          {/* <div className={colorcodeTrendRev(metrics?.sharesChangeTrend)}>
            {metrics?.sharesChangeTrend ?? "-"}
          </div> */}
        </div>
      )}
      <div className="flex flex-col col-span-full  bg-white rounded-xl sm:col-span-3 xl:col-span-2">
        <MetricsCardWithTwoIndicators
          title="Share Count Change (TTM)"
          value={`${metrics?.sharesChangeTtm?.toFixed(2) ?? "-"}%`}
          firstIndicatorTitle="4yr"
          firstIndicator={`${
            metrics?.sharesChangeMultiYear?.toFixed(2) ?? "-"
          }%`}
          firstClassName="text-sm text-black font-semibold px-0.5"
          secondIndicatorTitle="Long-term"
          secondIndicator={` ${metrics?.sharesChangeTrend ?? "-"}`}
          secondClassName={colorcodeTrendRev(metrics?.sharesChangeTrend)}
        />
        {/* <div className={shareDilutionoRedFlag(metrics?.sharesChangeTtm)}>
          {metrics?.sharesChangeTtm?.toFixed(2) ?? "-"}%
        </div> */}
      </div>
      {metrics?.retainedEarningsTtm && (
        <div className="flex flex-col col-span-full  bg-white rounded-xl sm:col-span-3 xl:col-span-2">
          <MetricsCardWithSingleIndicator
            title="Retained Earnings (TTM) in M"
            value={`${getCurrencySymbol(metrics?.currency)}${
              metrics?.retainedEarningsTtm ?? "-"
            }`}
            indicatorTitle="Long-term"
            indicator={metrics?.retainedEarningsTrend ?? "-"}
            className={colorcodeTrend(metrics?.retainedEarningsTrend)}
          />
        </div>
      )}
      {metrics?.netCashTtm && (
        <div className="flex flex-col col-span-full  bg-white rounded-xl sm:col-span-3 xl:col-span-2">
          <MetricsCardWithSingleIndicator
            title="Net Cash (TTM) in M"
            value={`${getCurrencySymbol(metrics?.currency)}${
              metrics?.netCashTtm ?? "-"
            }`}
            indicatorTitle="Long-term"
            indicator={metrics?.netCashTrend ?? "-"}
            className={colorcodeTrend(metrics?.netCashTrend)}
          />
        </div>
      )}
    </div>
  );
}

export default EarningsWidget;
