import SearchBar from "./SearchBar";
import TickerMetricsPage from "./TickerMetricsPage";
import { useState } from "react";
import { getMetrics } from "../api/bullseyeAPIv2";
import { Metrics } from "../api/Metrics";

function BullsEyeHome() {
  const [ticker, setTicker] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | undefined>(undefined);
  const [metrics, updateMetrics] = useState<Metrics>();
  async function loadMetrics(ticker: string) {
    setTicker(ticker);
    setLoading(true);
    try {
      const metrics = await getMetrics(ticker);
      updateMetrics(metrics);
      setError(undefined);
    } catch (e) {
      setError(e instanceof Error ? e.message : "Unknown error");
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="flex flex-col w-full">
      <SearchBar onSearch={(ticker) => loadMetrics(ticker)} />

      {ticker && metrics && (
        <TickerMetricsPage metrics={metrics} loading={loading} error={error} />
      )}
    </div>
  );
}
export default BullsEyeHome;
