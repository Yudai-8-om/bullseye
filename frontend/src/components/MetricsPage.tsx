import TickerMetricsPage from "./TickerMetricsPage";
import { Metrics } from "../api/Metrics";

interface MetricsPageProps {
  ticker: string | null;
  metrics: Metrics | undefined;
  loading: boolean;
  error: string | undefined;
}

function MetricsPage({ ticker, metrics, loading, error }: MetricsPageProps) {
  return (
    <div className="flex flex-col w-full">
      {ticker && (
        <TickerMetricsPage metrics={metrics} loading={loading} error={error} />
      )}
    </div>
  );
}
export default MetricsPage;
