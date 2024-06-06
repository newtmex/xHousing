import { useAccountTokens, useTokenPrices } from '@/hooks';
import { prettyFormatAmount } from '@/utils';
import { RoutePath } from '@/utils/routes';
import BigNumber from 'bignumber.js';
import Link from 'next/link';
import { useMemo } from 'react';

export default function PortfolioBalance() {
  const prices = useTokenPrices();
  const { xht, lkXht, otherTokens } = useAccountTokens();

  const portfolioValue = useMemo(() => {
    let value = new BigNumber(0);
    const addValue = <
      Bal extends { multipliedBy: any },
      N extends { balance: Bal; collection?: string; identifier?: string }
    >(
      n: N[] | N | null
    ) => {
      
      if (n) {
        if (n instanceof Array) {
          n.forEach((token) => {
            let price = prices[token.collection || token.identifier!];

            value = value.plus(
              token.balance
                .multipliedBy(new BigNumber(price.toFixed(2)))
                .dividedBy(1e2)
            );
          });
        } else {
          let price = prices[n.collection || n.identifier!];

          value = value.plus(
            n.balance.multipliedBy(new BigNumber(price.toFixed(2))).dividedBy(1e20)
          );
        }
      }
    };

    addValue(xht);
    addValue(lkXht);
    addValue(otherTokens);

    return prettyFormatAmount({ value: value.toFixed(0), decimals: 2 });
  }, [xht, lkXht, otherTokens, prices]);

  return (
    <div className='col-sm-12 col-lg-8 col-xxl-6'>
      <div className='element-balances justify-content-between mobile-full-width'>
        <div className='balance balance-v2'>
          <div className='balance-title'>Your Portfolio Balance</div>
          <div className='balance-value'>
            <span className='d-xxl-none'>${portfolioValue}</span>
            <span className='d-none d-xxl-inline-block'>${portfolioValue}</span>
          </div>
        </div>
        <div className='balance-table pl-sm-2'>
          <table className='table table-lightborder table-bordered table-v-compact mb-0'>
            <tbody>
              <tr>
                <td>
                  <strong>2.34</strong>
                  <div className='balance-label smaller lighter text-nowrap'>
                    Bitcoins BTC
                  </div>
                </td>
                <td>
                  <strong>1.74</strong>
                  <div className='balance-label smaller lighter text-nowrap'>
                    Ripple RPX
                  </div>
                </td>
                <td className='d-sm-none d-xxxxl-table-cell d-md-table-cell d-xxl-none'>
                  <strong>4.82</strong>
                  <div className='balance-label smaller lighter text-nowrap'>
                    Newcoin NCN
                  </div>
                </td>
              </tr>
              <tr>
                <td>
                  <strong>1.22</strong>
                  <div className='balance-label smaller lighter text-nowrap'>
                    Litecoin LTC
                  </div>
                </td>
                <td>
                  <strong>1.87</strong>
                  <div className='balance-label smaller lighter text-nowrap'>
                    Etherium ETH
                  </div>
                </td>
                <td className='d-sm-none d-xxxxl-table-cell d-md-table-cell d-xxl-none'>
                  <strong>1.02</strong>
                  <div className='balance-label smaller lighter text-nowrap'>
                    Dogecoin DGC
                  </div>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
      <div className='element-wrapper pb-4 mb-4 border-bottom'>
        <div className='element-box-tp'>
          <Link className='btn btn-primary' href={RoutePath.Properties}>
            <i className='os-icon os-icon-plus-circle'></i>
            <span>Buy Property</span>
          </Link>
        </div>
      </div>
    </div>
  );
}
