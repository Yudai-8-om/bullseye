export function getCurrencySymbol(currencyCode: string): string {
  const symbols: Record<string, string> = {
    USD: "$", // US Dollar
    EUR: "€", // Euro
    GBP: "£", // British Pound
    JPY: "¥", // Japanese Yen
    CHF: "CHF", // Swiss Franc
    AUD: "A$", // Australian Dollar
    CAD: "C$", // Canadian Dollar
    CNY: "¥", // Chinese Yuan
    INR: "₹", // Indian Rupee
    KRW: "₩", // South Korean Won
    SEK: "kr", // Swedish Krona
    NOK: "kr", // Norwegian Krone
    MXN: "$", // Mexican Peso
    BRL: "R$", // Brazilian Real
    ZAR: "R", // South African Rand
  };

  return symbols[currencyCode.toUpperCase()] || currencyCode;
}
