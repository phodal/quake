// String
Object.defineProperty(String.prototype, 'uppercase', {
  value() {
    return this.toUpperCase();
  }
});
Object.defineProperty(String.prototype, 'lowercase', {
  value() {
    return this.toLowerCase();
  }
});

// date
Object.defineProperty(String.prototype, 'date', {
  value() {
    return new Date(this);
  }
});

// number
Object.defineProperty(String.prototype, 'float', {
  value() {
    return parseFloat(this);
  }
});
Object.defineProperty(String.prototype, 'int', {
  value() {
    return parseInt(this);
  }
});

// convert data
Object.defineProperty(Number.prototype, 'sqrt', {
  value() {
    return Math.sqrt(this);
  }
});
Object.defineProperty(Number.prototype, 'sin', {
  value() {
    return Math.sin(this);
  }
});
Object.defineProperty(Number.prototype, 'cos', {
  value() {
    return Math.cos(this);
  }
});
Object.defineProperty(Number.prototype, 'fixed', {
  value(num) {
    return this.toFixed(num);
  }
});
