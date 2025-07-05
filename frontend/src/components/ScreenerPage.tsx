import { Metrics } from "../api/Metrics";
import { getAllMetrics } from "../api/bullseyeAPIv2";
import CompanyList from "./CompanyList";
import { useState, useEffect } from "react";

function ScreenerPage() {
  const [allMetrics, updateAllMetrics] = useState<Metrics[]>();

  async function getCompanyList() {
    const allMetrics = await getAllMetrics();
    updateAllMetrics(allMetrics);
  }
  useEffect(() => {
    getCompanyList();
  }, []);
  return (
    <div className="flex flex-col w-full">
      {allMetrics && <CompanyList metrics={allMetrics} />}
    </div>
  );
}

export default ScreenerPage;
