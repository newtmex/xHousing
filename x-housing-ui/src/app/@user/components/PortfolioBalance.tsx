import { useXProjects } from '@/contracts/xProject/hooks';
import { useAccountTokens, useTokenPrices } from '@/hooks';
import { prettyFormatAmount } from '@/utils';
import { RoutePath } from '@/utils/routes';
import { NonFungibleTokenOfAccountOnNetwork } from '@multiversx/sdk-network-providers/out';
import BigNumber from 'bignumber.js';
import Link from 'next/link';
import { useMemo } from 'react';

export default function PortfolioBalance() {
  const prices = useTokenPrices();
  const { xht, lkXht, xProjectsToken } = useAccountTokens();
  const xProjectsTokenDetail = useXProjects().map((v) => ({
    collection: v.projectData.tokenId,
    name: v.projectData.tokenInfo.name
  }));

  const portfolioValue = useMemo(() => {
    let value = new BigNumber(0);

    for (const token of [xht, lkXht, ...(xProjectsToken || [])]) {
      if (token) {
        const price =
          prices[
            (token as NonFungibleTokenOfAccountOnNetwork).collection ||
              token.identifier
          ];

        value = value.plus(
          token.balance
            .multipliedBy(new BigNumber(price.toFixed(2)))
            .dividedBy(1e20)
        );
      }
    }

    return prettyFormatAmount({ value: value.toFixed(0), decimals: 2 });
  }, [xht, lkXht, xProjectsToken, prices]);

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
                {xProjectsTokenDetail.map((token) => (
                  <td key={token.collection}>
                    <strong>${prices[token.collection].toFixed(2)}</strong>
                    <div className='balance-label smaller lighter text-nowrap'>
                      {token.name} {token.collection}
                    </div>
                  </td>
                ))}
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
