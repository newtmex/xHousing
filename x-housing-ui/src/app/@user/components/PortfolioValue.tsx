'use client';

import { useXhtID } from '@/contracts/coinbase/hooks';
import { xProjectFundingSC } from '@/contracts/xProjectFunding';
import { useAccountTokens, useGetAccount } from '@/hooks';
import { getWindowLocation, prettyFormatAmount } from '@/utils';
import { RoutePath } from '@/utils/routes';
import { signAndSendTransactions } from '@/utils/signAndSendTransactions';
import Link from 'next/link';
import { usePathname } from 'next/navigation';
import { useCallback, useEffect, useMemo, useRef, useState } from 'react';

const usePortfolioViewToggler = () => {
  const [opened, setOpened] = useState(false);

  const pathname = usePathname();
  const pathnameRef = useRef(pathname);
  useEffect(() => {
    if (pathname !== pathnameRef.current) {
      setOpened(false);
      pathnameRef.current = pathname;
    }
  }, [pathname]);

  return {
    opened,
    viewToggler: () => {
      // Toggle opened state
      setOpened(!opened);
    }
  };
};

export default function PortfolioValue() {
  const { opened, viewToggler } = usePortfolioViewToggler();
  const { xht, lkXht, xProjectsToken } = useAccountTokens();
  const xhtID = useXhtID();
  const pathname = usePathname();

  const { address } = useGetAccount();

  const onUnlockLockedXHT = useCallback(async () => {
    if (!lkXht) {
      return;
    }

    let unlockLkXHTtx = xProjectFundingSC.makeUnlockXhtTx({
      token: lkXht,
      address
    });
    unlockLkXHTtx.sender = address;

    console.log('sending', unlockLkXHTtx.getData().toString());

    await signAndSendTransactions({
      transactions: [unlockLkXHTtx],
      callbackRoute: getWindowLocation().pathname,
      transactionsDisplayInfo: {}
    });
  }, [lkXht, address]);

  return (
    <div className={`fancy-selector-w ${opened ? 'opened' : ''}`}>
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
        <div onClick={viewToggler} className='fs-selector-trigger'>
          <i className='os-icon os-icon-arrow-down4'></i>
        </div>
      </div>
      <div className='fancy-selector-options'>
        {lkXht && (
          <div className='fancy-selector-option'>
            <div className='fs-img'>
              <img alt='' src='img/card2.png' />
            </div>
            <div className='fs-main-info'>
              <div className='fs-name'>
                <span>{lkXht.name} Portfolio</span>
                <strong>{lkXht.collection}</strong>
              </div>
              <div className='fs-sub'>
                <span>Balance:</span>
                <strong>
                  {prettyFormatAmount({ value: lkXht.balance.toFixed(0) })}
                </strong>
              </div>
            </div>
            <button
              onClick={() => onUnlockLockedXHT()}
              className='btn btn-primary'
              style={{ fontSize: '0.75em' }}
            >
              Unlock
            </button>
          </div>
        )}

        {xProjectsToken && <>Properties</>}
        {xProjectsToken?.map((token) => (
          <div className='fancy-selector-option'>
            <div className='fs-img'>
              <img alt='' src='img/card2.png' />
            </div>
            <div className='fs-main-info'>
              <div className='fs-name'>
                <span>{token.name} Portfolio</span>
                <strong>{token.collection}</strong>
              </div>
              <div className='fs-sub'>
                <span>Units:</span>
                <strong>
                  {prettyFormatAmount({
                    value: token.balance.toFixed(0),
                    decimals: 0
                  })}
                </strong>
              </div>
            </div>
          </div>
        ))}

        {!pathname.includes(RoutePath.Properties) && (
          <div className='fancy-selector-actions text-right'>
            <Link className='btn btn-primary' href={RoutePath.Properties}>
              <i className='os-icon os-icon-ui-22'></i>
              <span>Add Property</span>
            </Link>
          </div>
        )}
      </div>
    </div>
  );
}
