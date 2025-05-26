export const baseUrl = "http://localhost:3000";

function translateStatusToMessage(status: number) {
  switch (status) {
    case 401:
      return "please login again.";
    default:
      return "Please try again";
  }
}
function checkStatus(response: any) {
  if (response.ok) {
    return response;
  } else {
    const httpErrorInfo = {
      status: response.status,
      statusText: response.statusText,
      url: response.url,
    };
    let errorMessage = translateStatusToMessage(httpErrorInfo.status);
    throw new Error(errorMessage);
  }
}

function parseJSON(response: Response) {
  return response.json();
}
function delay(ms: number) {
  return function (x: any): Promise<any> {
    return new Promise((resolve) => setTimeout(() => resolve(x), ms));
  };
}
const bullseyeAPI = {
  get(ticker: string) {
    const url = `${baseUrl}/search/${ticker.toLowerCase()}`;
    return fetch(url)
      .then(delay(600))
      .then(checkStatus)
      .then(parseJSON)
      .catch((error) => {
        console.log("log client error " + error);
        throw new Error(
          "There was an error retrieving the projects. Please try again."
        );
      });
  },
};

export { bullseyeAPI };
