export * from './sdkDappUtils';
export * from './operations';

export const isAuthRoute = (path: string) => {
  return path.startsWith('/unlock');
};

/**
 * Truncates a string from characters within the string
 * @param string The string to truncate from inside
 * @param length Total length of the truncated string
 * @returns Truncated string of length
 */
export function truncateFromInside(string: string, length: number = 5): string {
  if (length < 5) throw 'Truncated string length must be >= 5';
  if (string.length <= length) return string;

  // remove the ellipses
  const finalLength = length - 3;

  const remChar = finalLength % 2;
  const eachSide = Math.floor(finalLength / 2);

  return (
    string.slice(0, eachSide) + '...' + string.slice(-(eachSide + remChar))
  );
}

export async function sleep<R>(secs: number, cb?: () => R): Promise<R | null> {
  return await new Promise((resolve) =>
    setTimeout(() => resolve(cb ? cb() : null), Math.floor(secs * 1000))
  );
}
