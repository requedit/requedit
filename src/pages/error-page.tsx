import { useRouteError } from "react-router-dom";

interface RouteError extends Error {
  statusText: string;
}
export default function ErrorPage() {
  const error = useRouteError() as RouteError;

  return (
    <div id="error-page" className="flex flex-col justify-center items-center">
      <h1 className="text-2xl mb-2 p-3">Oops!</h1>
      <p>Sorry, an unexpected error has occurred.</p>
      <p>
        <i>{error.statusText || error.message}</i>
      </p>
    </div>
  );
}
