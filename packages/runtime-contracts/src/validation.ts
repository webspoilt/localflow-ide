import { z, ZodError } from 'zod';

export function validateOrThrow<T>(schema: z.ZodSchema<T>, data: unknown): T {
  const result = schema.safeParse(data);
  if (!result.success) {
    throw new ValidationError(result.error);
  }
  return result.data;
}

export function validate<T>(schema: z.ZodSchema<T>, data: unknown): {
  success: boolean;
  data?: T;
  error?: ZodError;
} {
  const result = schema.safeParse(data);
  if (result.success) {
    return { success: true, data: result.data };
  }
  return { success: false, error: result.error };
}

export class ValidationError extends Error {
  public readonly zodError: ZodError;

  constructor(zodError: ZodError) {
    super(`Validation failed: ${zodError.message}`);
    this.name = 'ValidationError';
    this.zodError = zodError;
  }

  get issues() {
    return this.zodError.issues.map((issue) => ({
      path: issue.path.join('.'),
      message: issue.message,
      code: issue.code,
    }));
  }
}
