export const MethodTypes = {
    get: "GET",
    post: "POST",
    put: "PUT",
    delete: "DELETE"
} as const;

export type MethodType = typeof MethodTypes[keyof typeof MethodTypes];