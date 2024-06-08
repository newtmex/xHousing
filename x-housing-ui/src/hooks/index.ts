import { useXhtID } from '@/contracts/coinbase/hooks';
import { useXProjects } from '@/contracts/xProject/hooks';
import { useLkXhtID } from '@/contracts/xProjectFunding/hooks';
import { useEffect, useMemo, useRef, useState } from 'react';

import { useGetAccount } from '@multiversx/sdk-dapp/hooks/account/useGetAccount';
import {
  FungibleTokenOfAccountOnNetwork,
  NonFungibleTokenOfAccountOnNetwork
} from '@multiversx/sdk-network-providers/out';

import { apiProvider } from '@/providers/apiProvider';

export * from './useWindowResize';
export * from './sdkDappHooks';
export * from './useGlobalData';

import useSWR from 'swr';
import { usePathname } from 'next/navigation';

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
      const xProjectsToken =
        nfts.status == 'rejected'
          ? null
          : nfts.value.filter((token) => {
              if (lkXhtID == token.collection) {
                userLkXht = token;
                return false;
              }

              return xProjectsTokenId.includes(token.collection);
            });

      return [userXht, userLkXht, xProjectsToken] as [
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
      xProjectsToken: null
    };
  }

  const [xht, lkXht, xProjectsToken] = data;

  return { xht, lkXht, xProjectsToken };
};

// TODO get real values
export const useTokenPrices = () => {
  const xProjectsTokenId = useXProjects().map((v) => v.projectData.tokenId);
  const xhtID = useXhtID();
  const lkXhtID = useLkXhtID();

  return useMemo(() => {
    const prices: { [key: string]: number } = {};

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

export const useContentPanel = () => {
  const { data, mutate } = useSWR('ui-contentent-panel', {
    fallbackData: false
  });

  useEffect(() => {}, []);

  return {
    contentPanelActive: data,
    toggleContentPanel() {
      mutate(!data);
    },
    hideContentPanel() {
      console.log('hide');
      mutate(false);
    },
    showContentPanel() {
      mutate(true);
    }
  };
};

export const useOnPathChange = (cb: () => void) => {
  const pathname = usePathname();
  const pathnameRef = useRef(pathname);
  useEffect(() => {
    if (pathname !== pathnameRef.current) {
      cb();
      pathnameRef.current = pathname;
    }
  }, [pathname, cb]);
};
