import { JSX } from "react";
import { Metrics } from "../api/Metrics";

interface CompanyListProps {
  metrics: Metrics[];
}
function formatTrend(trend: string | undefined): JSX.Element {
  switch (trend) {
    case "Downtrend":
      return (
        <div className="w-7 h-5 bg-red-500/50 transform rounded-full">
          {trend}
        </div>
      );
    case "Uptrend":
      return (
        <div className="w-7 h-5 bg-green-500/50 transform rounded-full">
          {trend}
        </div>
      );
    default:
      return <div>{trend}</div>;
  }
}

function formatTrendRev(trend: string | undefined): JSX.Element {
  switch (trend) {
    case "Downtrend":
      return (
        <div className="w-7 h-5 bg-green-500/50 transform rounded-full">
          {trend}
        </div>
      );
    case "Uptrend":
      return (
        <div className="w-7 h-5 bg-red-500/50 transform rounded-full">
          {trend}
        </div>
      );
    default:
      return <div>{trend}</div>;
  }
}

function CompanyList({ metrics }: CompanyListProps) {
  return (
    <div className="flex flex-col w-full bg-gray-900 overflow-x-auto">
      <h1 className="text-3xl font-bold ml-5 m-3 text-gray-200">Screener</h1>
      <table className="m-2 text-sm font-semibold text-gray-300">
        <thead className="bg-gray-700">
          <tr>
            <th className="px-3 py-3">Ticker</th>
            <th className="px-3 py-3">Company Name</th>
            <th className="px-3 py-3">Revenue Growth</th>
            <th className="px-3 py-3">Gross Margin</th>
            <th className="px-3 py-3">Operating Margin</th>
            <th className="px-3 py-3">Net Margin</th>
            <th className="px-3 py-3">Gross Margin Short Term</th>
            <th className="px-3 py-3">Gross Margin Long Term</th>
            <th className="px-3 py-3">SGA Short Term</th>
            <th className="px-3 py-3">SGA Long Term</th>
            <th className="px-3 py-3">R&D Short Term</th>
            <th className="px-3 py-3">R&D Long Term</th>
            <th className="px-3 py-3">Operating Margin Short Term</th>
            <th className="px-3 py-3">Operating Margin Long Term</th>
          </tr>
        </thead>
        <tbody>
          {metrics.map((metric) => (
            <tr className="bg-gray-800 border-b border-gray-500 text-gray-50 hover:bg-gray-600">
              <td className="px-3 py-4 text-base">
                {metric.ticker.toUpperCase()}
              </td>
              <td className="px-3 py-4">{metric.companyName}</td>
              <td className="px-3 py-4">{metric.revenueGrowthYoyTtm}%</td>
              <td className="px-3 py-4">{metric.grossMarginTtm}%</td>
              <td className="px-3 py-4">{metric.operatingMarginTtm}%</td>
              <td className="px-3 py-4">{metric.netMarginTtm}%</td>
              <td className="px-3 py-4">
                {formatTrend(metric.grossMarginShortTermTrend)}
              </td>
              <td className="px-3 py-4">
                {formatTrend(metric.grossMarginLongTermTrend)}
              </td>
              <td className="px-3 py-4">
                {formatTrendRev(metric.sgaShortTermTrend)}
              </td>
              <td className="px-3 py-4">
                {formatTrendRev(metric.sgaLongTermTrend)}
              </td>
              <td className="px-3 py-4">
                {formatTrendRev(metric.rndShortTermTrend)}
              </td>
              <td className="px-3 py-4">
                {formatTrendRev(metric.rndLongTermTrend)}
              </td>
              <td className="px-3 py-4">
                {formatTrend(metric.operatingMarginShortTermTrend)}
              </td>
              <td className="px-3 py-4">
                {formatTrend(metric.operatingMarginLongTermTrend)}
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export default CompanyList;
