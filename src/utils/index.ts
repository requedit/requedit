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
