

export function getLoginUrl() {
  return `http://127.0.0.1:9900/api/auth/login?redirect_uri=${`http://127.0.0.1:3000`}`
}
