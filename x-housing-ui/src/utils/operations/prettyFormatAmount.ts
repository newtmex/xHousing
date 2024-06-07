import BigNumber from 'bignumber.js';
import { DECIMALS } from '@/constants';
import { formatAmount } from '../sdkDappUtils';

BigNumber.config({ ROUNDING_MODE: BigNumber.ROUND_FLOOR });

export function prettyFormatAmount({
  value,
  length = 8,
  minLength = 30,
  decimals = DECIMALS,
  showIsLessThanDecimalsLabel = true
}: {
  value: string;
  length?: number;
  minLength?: number;
  decimals?: number;
  showIsLessThanDecimalsLabel?: boolean;
}) {
  const digits =
    value.length <= minLength ? length : length - (value.length - minLength);
  return formatAmount({
    input: value,
    digits: digits <= 0 ? 0 : digits,
    showLastNonZeroDecimal: false,
    showIsLessThanDecimalsLabel,
    addCommas: true,
    decimals
  });
}
