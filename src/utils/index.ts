export function parseHttpStatus(res?: Record<string, any>) {
  if (!res) return null;
  const [codeStr] = res.status.split(" ");
  const code = parseInt(codeStr, 10);

  return isNaN(code) ? null : code;
}

export const getStatusColor = (res: any): string => {
  if (!res) return "gray";
  const statuscode = parseHttpStatus(res);
  if (String(statuscode).startsWith("1")) return "#2196f3";
  if (String(statuscode).startsWith("2")) return "green";
  if (String(statuscode).startsWith("3")) return "gold";
  return String(statuscode).startsWith("4") ||
    String(statuscode).startsWith("5")
    ? "red"
    : "gray";
};

export function buildCurlCommand(
  url: string,
  method: string = 'GET',
  headers: { [key: string]: string } = {},
  body: any
): string {
  let curlCmd = `curl -X ${method.toUpperCase()} "${url}"`;

  for (const [key, value] of Object.entries(headers)) {
      curlCmd += ` \\\n  -H '${key}: ${value}'`;
  }

  if (body && (method === 'POST' || method === 'PUT')) {
      const data = typeof body === 'string' ? body : JSON.stringify(body);
      curlCmd += ` \\\n  -d '${data}'`;
  }

  return curlCmd;
}
