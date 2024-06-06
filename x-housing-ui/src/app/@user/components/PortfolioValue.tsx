import { useXhtID } from '@/contracts/coinbase/hooks';
import { useXProjects } from '@/contracts/xProject/hooks';
import { useLkXhtID } from '@/contracts/xProjectFunding/hooks';
import { apiProvider } from '@/providers/apiProvider';
import { prettyFormatAmount } from '@/utils';
import { useGetAccount } from '@multiversx/sdk-dapp/hooks/account/useGetAccount';
import {
  FungibleTokenOfAccountOnNetwork,
  NonFungibleTokenOfAccountOnNetwork
} from '@multiversx/sdk-network-providers/out';
import { useMemo } from 'react';
import useSWR from 'swr';

const useAccountTokens = () => {
  const xProjectsTokenId = useXProjects().map((v) => v.projectData.tokenId);
  const xhtID = useXhtID();
  const lkXhtID = useLkXhtID();

  const { address } = useGetAccount();

  return useSWR(
    xhtID && lkXhtID ? { xProjectsTokenId, address, xhtID, lkXhtID } : null,
    async ({ address, xProjectsTokenId, xhtID, lkXhtID }) => {
      const [xht, nfts] = await Promise.allSettled([
        apiProvider.getFungibleTokenOfAccount({ bech32: () => address }, xhtID),

        apiProvider.getNonFungibleTokensOfAccount({ bech32: () => address })
      ]);

      const userXht = xht.status == 'fulfilled' ? xht.value : null;
      let userLkXht: NonFungibleTokenOfAccountOnNetwork | null = null;
      const otherTokens =
        nfts.status == 'rejected'
          ? null
          : nfts.value.filter((token) => {
              if (lkXhtID == token.collection) {
                userLkXht = token;
                return false;
              }

              return xProjectsTokenId.includes(token.collection);
            });

      return [userXht, userLkXht, otherTokens] as [
        FungibleTokenOfAccountOnNetwork | null,
        NonFungibleTokenOfAccountOnNetwork | null,
        NonFungibleTokenOfAccountOnNetwork[] | null
      ];
    }
  ).data;
};

export default function PortfolioValue() {
  const accountTokens = useAccountTokens();
  const xhtID = useXhtID();

  if (!accountTokens) {
    return null;
  }

  const [xht, lkXht, otherTokens] = accountTokens;

  console.log({ xht, lkXht, otherTokens });

  return (
    <div className='fancy-selector-w'>
      <div className='fancy-selector-current'>
        <div className='fs-img'>
          <img alt='' src='img/card4.png' />
        </div>
        <div className='fs-main-info'>
          <div className='fs-name'>
            <span>xHousing Portfolio</span>
            <strong>{xhtID}</strong>
          </div>
          <div className='fs-sub'>
            <span>Balance:</span>
            <strong>
              {prettyFormatAmount({ value: xht?.balance.toFixed(0) || '0' })}
            </strong>
          </div>
        </div>
        <div className='fs-selector-trigger'>
          <i className='os-icon os-icon-arrow-down4'></i>
        </div>
      </div>
      <div className='fancy-selector-options'>
        <div className='fancy-selector-option'>
          <div className='fs-img'>
            <img alt='' src='img/card2.png' />
          </div>
          <div className='fs-main-info'>
            <div className='fs-name'>
              <span>Lite Portfolio</span>
              <strong>ETH</strong>
            </div>
            <div className='fs-sub'>
              <span>Balance:</span>
              <strong>$5,304</strong>
            </div>
          </div>
        </div>
        <div className='fancy-selector-option active'>
          <div className='fs-img'>
            <img alt='' src='img/card4.png' />
          </div>
          <div className='fs-main-info'>
            <div className='fs-name'>
              <span>Bitcoin Portfolio</span>
              <strong>BTC</strong>
            </div>
            <div className='fs-sub'>
              <span>Balance:</span>
              <strong>$8,274</strong>
            </div>
          </div>
        </div>
        <div className='fancy-selector-option'>
          <div className='fs-img'>
            <img alt='' src='img/card3.png' />
          </div>
          <div className='fs-main-info'>
            <div className='fs-name'>
              <span>Ripple Portfolio</span>
              <strong>RPX</strong>
            </div>
            <div className='fs-sub'>
              <span>Balance:</span>
              <strong>$1,202</strong>
            </div>
          </div>
        </div>
        <div className='fancy-selector-actions text-right'>
          <a className='btn btn-primary' href='#'>
            <i className='os-icon os-icon-ui-22'></i>
            <span>Add Account</span>
          </a>
        </div>
      </div>
    </div>
  );
}
