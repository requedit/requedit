export function parseHttpStatus(res?: Record<string, any>) {
  if (!res) return null;
  const [codeStr] = res.status.split(" ");
  const code = parseInt(codeStr, 10);

  return isNaN(code) ? null : code;
}
