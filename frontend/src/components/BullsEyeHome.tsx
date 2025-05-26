import SearchBar from "./SearchBar";
import TickerMetricsPage from "./TickerMetricsPage";
import { useState } from "react";
import { getMetrics } from "../api/bullseyeAPIv2";
import { NominalMetrics } from "../api/nominalMetrics";

function BullsEyeHome() {
  const [ticker, setTicker] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | undefined>(undefined);
  const [nominalMetrics, updateMetrics] = useState<NominalMetrics>();
  async function loadMetrics(ticker: string) {
    setTicker(ticker);
    setLoading(true);
    try {
      const nominalMetrics = await getMetrics(ticker);
      updateMetrics(nominalMetrics);
      setError(undefined);
    } catch (e) {
      setError(e instanceof Error ? e.message : "Unknown error");
    } finally {
      setLoading(false);
    }
  }

  return (
    <>
      <header>
        <SearchBar onSearch={(ticker) => loadMetrics(ticker)} />
      </header>
      {ticker && (
        <TickerMetricsPage
          nominalMetrics={nominalMetrics}
          loading={loading}
          error={error}
        />
      )}
    </>
  );
}
export default BullsEyeHome;
