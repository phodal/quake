function padLeft(nr, n = 4, str) {
  return Array(n - String(nr).length + 1).join(str || '0') + nr;
}

export default padLeft;
