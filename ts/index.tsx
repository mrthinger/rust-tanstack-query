import createFetchClient from "openapi-fetch";
import createClient from "openapi-react-query";
import type { paths } from "./api.js"; // generated by openapi-typescript

const fetchClient = createFetchClient<paths>({
  baseUrl: "https://myapi.dev/v1/",
});
const $api = createClient(fetchClient);

const MyComponent = () => {
  const { data, error, isLoading } = $api.useQuery("get", "/trade", {});

  if (isLoading || !data) return "Loading...";

  if (error) return `An error occurred: ${error}`;

  // data.levels
  // data.side

  return <div>{JSON.stringify(data)}</div>;
};
