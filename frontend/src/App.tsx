import MetricsPage from "./components/MetricsPage";
import ScreenerPage from "./components/ScreenerPage";
import SearchBar from "./components/SearchBar";
import { Routes, Route, NavLink } from "react-router-dom";
import { useState } from "react";
import { getMetrics } from "./api/bullseyeAPIv2";
import { Metrics } from "./api/Metrics";

function App() {
  const [ticker, setTicker] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | undefined>(undefined);
  const [metrics, updateMetrics] = useState<Metrics | undefined>(undefined);
  async function loadMetrics(ticker: string) {
    setLoading(true);
    setTicker(ticker);
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
    <>
      <header className="bg-green-400 py-4 h-20 flex items-center justify-around">
        <div className="font-bold text-2xl">BullsEye</div>
        <NavLink
          to="/"
          className={({ isActive }) =>
            isActive
              ? "text-sm/6 font-bold text-green-900 no-underline border-blue-600"
              : "text-sm/6 font-semibold text-gray-900 no-underline hover:text-blue-600"
          }
        >
          Screener
        </NavLink>
        <NavLink
          to="/search"
          className={({ isActive }) =>
            isActive
              ? "text-sm/6 font-bold text-green-900 no-underline  border-blue-600"
              : "text-sm/6 font-semibold text-gray-900 no-underline hover:text-blue-600"
          }
        >
          Search
        </NavLink>
        <SearchBar onSearch={(ticker) => loadMetrics(ticker)} />
      </header>
      <div className="bg-gray-200 min-h-screen w-full">
        <Routes>
          <Route path="/" element={<ScreenerPage />} />
          <Route
            path="/search"
            element={
              <MetricsPage
                ticker={ticker}
                metrics={metrics}
                loading={loading}
                error={error}
              />
            }
          />
        </Routes>
      </div>
    </>
  );
}

export default App;
