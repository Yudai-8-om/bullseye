import { Metrics } from "../api/Metrics";

const baseUrl = "http://localhost:3000";

export async function getMetrics(ticker: string): Promise<Metrics> {
  const url = `${baseUrl}/companies/${ticker.toLowerCase()}`;
  try {
    const metrics = await fetchData<Metrics>(url);
    console.log(JSON.stringify(metrics, null, 2));
    return metrics;
  } catch (error) {
    if (error instanceof Error) {
      throw new Error(
        `There was an error retrieving data. Please try again. \n${error.message}`
      );
    } else {
      throw new Error(
        "There was an error retrieving data due to unexpected error. Please contact the developer."
      );
    }
  }
}

async function fetchData<T>(url: string, options?: RequestInit): Promise<T> {
  const response = await fetch(url, options);
  if (!response.ok) {
    const errorText = await response.text();
    throw new Error(`Status: ${response.status}, Message: ${errorText}`);
  } else {
    return parseJSON<T>(response);
  }
}

function parseJSON<T>(response: Response): Promise<T> {
  return response.json();
}
