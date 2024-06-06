import { useXhtID } from '@/contracts/coinbase/hooks';
import { useXProjects } from '@/contracts/xProject/hooks';
import { useLkXhtID } from '@/contracts/xProjectFunding/hooks';
import { useMemo } from 'react';

import { useGetAccount } from '@multiversx/sdk-dapp/hooks/account/useGetAccount';
import {
  FungibleTokenOfAccountOnNetwork,
  NonFungibleTokenOfAccountOnNetwork
} from '@multiversx/sdk-network-providers/out';

import { apiProvider } from '@/providers/apiProvider';
export * from './sdkDappHooks';
import useSWR from 'swr';

export const useAccountTokens = () => {
  const xProjectsTokenId = useXProjects().map((v) => v.projectData.tokenId);
  const xhtID = useXhtID();
  const lkXhtID = useLkXhtID();

  const { address } = useGetAccount();

  const { data } = useSWR(
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
  );

  if (!data) {
    return {
      xht: null,
      lkXht: null,
      otherTokens: null
    };
  }

  const [xht, lkXht, otherTokens] = data;

  return { xht, lkXht, otherTokens };
};

// TODO get real values
export const useTokenPrices = () => {
  const xProjectsTokenId = useXProjects().map((v) => v.projectData.tokenId);
  const xhtID = useXhtID();
  const lkXhtID = useLkXhtID();

  return useMemo(() => {
    let prices: { [key: string]: number } = {};

    if (xhtID) {
      prices[xhtID] = Math.random() * 10;
    }
    if (lkXhtID) {
      prices[lkXhtID] = Math.random() * 10;
    }

    xProjectsTokenId.forEach((tokenId) => {
      prices[tokenId] = Math.random() * 10;
    });

    return prices;
  }, [xhtID, xProjectsTokenId, lkXhtID]);
};