const buildNestedWithServiceQuery = (services: any[]): string => {
  let query = "id stdout";
  for (let i = services.length - 1; i >= 0; i--) {
    const service = services[i];
    query = `
      withService(service: {${buildParams(service)}}) {
        ${query}
      }
    `;
  }
  return query;
};

const buildParams = (params: any): string => {
  let query = "";
  for (const key in params) {
    const value = params[key];

    if (params[key] === undefined) continue;

    if (Array.isArray(value)) {
      query += `${key}: [${value.map((v) => `"${v}"`).join(", ")}], `;
    }

    if (typeof value === "object" && !Array.isArray(value)) {
      const array = Object.entries(value).map(
        ([key, value]) => `"${key}=${value}"`
      );
      query += `${key}: [${array.join(", ")}], `;
    }

    if (typeof value === "number") {
      query += `${key}: ${value}, `;
    }
    if (typeof value === "string") {
      query += `${key}: "${value}", `;
    }
  }
  return query.slice(0, -2);
};

export { buildNestedWithServiceQuery };
