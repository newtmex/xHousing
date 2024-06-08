'use client';

import { useXhtID } from '@/contracts/coinbase/hooks';
import { XProjectsValue, useXProjects } from '@/contracts/xProject/hooks';
import { xProjectFundingSC } from '@/contracts/xProjectFunding';
import { useAccountTokens } from '@/hooks';
import { getItem } from '@/storage/session';
import { RefIdData, getWindowLocation, prettyFormatAmount } from '@/utils';
import { RoutePath } from '@/utils/routes';
import { signAndSendTransactions } from '@/utils/signAndSendTransactions';
import { TokenTransfer, Transaction } from '@multiversx/sdk-core/out';
import { useGetAccount } from '@multiversx/sdk-dapp/hooks/account/useGetAccount';
import BigNumber from 'bignumber.js';
import Link from 'next/link';
import { useCallback } from 'react';

export default function Properties() {
  const referrerKey = getItem('userRefBy');
  const referrerID = referrerKey ? RefIdData.getID(referrerKey) : undefined;
  const properties = useXProjects();

  const { address: loggedInUserAdsress } = useGetAccount();
  const { xProjectsToken: _xProjectsToken, xht } = useAccountTokens();

  const xProjectsToken = _xProjectsToken?.map((token) => token.collection);

  const xhtId = useXhtID();

  const onRentProperty = useCallback(
    async ({ contract }: Pick<XProjectsValue['projectData'], 'contract'>) => {
      if (!xhtId || !xht) {
        return;
      }

      const tx = contract.makeReceiveRentTx({
        payment: TokenTransfer.fungibleFromAmount(
          xhtId,
          new BigNumber(xht.balance.dividedBy(3)),
          0
        )
      });

      tx.sender = loggedInUserAdsress;

      const transactions: Transaction[] = [tx];

      await signAndSendTransactions({
        transactions,
        callbackRoute: getWindowLocation().pathname,
        transactionsDisplayInfo: {}
      });
    },
    [xhtId, loggedInUserAdsress, xht]
  );

  const onBuyPropertyUnits = useCallback(
    async ({ data }: Pick<XProjectsValue['projectData'], 'data'>) => {
      const transactions: Transaction[] = [];

      if (!data.isTokensClaimable) {
        const fundProjectTx = xProjectFundingSC.makeFundProjectTx({
          projectID: data.id,
          amount: new BigNumber(
            data.funding_goal.multipliedBy(5).dividedBy(2).toFixed(0)
          ),
          referrerID,
          sender: loggedInUserAdsress
        });

        transactions.push(fundProjectTx);
      }
      if (data.isTokensClaimable) {
        const claimXProjectTokenTx = xProjectFundingSC.makeClaimXProjectTokenTx(
          {
            projectID: data.id,
            sender: loggedInUserAdsress
          }
        );

        transactions.push(claimXProjectTokenTx);
      }

      await signAndSendTransactions({
        transactions,
        callbackRoute: getWindowLocation().pathname,
        transactionsDisplayInfo: {}
      });
    },
    [loggedInUserAdsress, referrerID]
  );

  return (
    <div className='all-wrapper rentals'>
      <div className='rentals-list-w hide-filters'>
        <div className='rentals-list'>
          <div className='property-items as-grid'>
            {!properties.length ? (
              <div style={{ height: '100%', fontSize: '10rem' }}>
                No Listed Properties
              </div>
            ) : (
              properties.map(
                ({
                  projectData: { tokenId, data, contract },
                  description,
                  features,
                  rentPrice,
                  unitPrice,
                  image
                }) => {
                  const href = `${RoutePath.Properties}`;

                  return (
                    <div className='property-item' key={`property-${data.id}`}>
                      <Link className='item-media-w' href={href}>
                        <div
                          className='item-media'
                          style={{ backgroundImage: `url(${image})` }}
                        ></div>
                      </Link>
                      <div className='item-info'>
                        <div className='item-features'>
                          {features.map((feature) => (
                            <div key={feature} className='feature'>
                              {feature}
                            </div>
                          ))}
                        </div>
                        <h3 className='item-title'>
                          <Link href={href}>{description}</Link>
                        </h3>

                        <div className='item-price-buttons row'>
                          {data.isTokensClaimable && (
                            <div className='col-12 row'>
                              <div className='item-price col-8'>
                                <strong>${rentPrice}</strong>
                                <span>/per year</span>
                              </div>
                              <div className='item-buttons col-4'>
                                <button
                                  onClick={() => onRentProperty({ contract })}
                                  className='btn btn-primary'
                                >
                                  Rent
                                </button>
                              </div>
                            </div>
                          )}
                          <div className='col-12 row'>
                            {!data.isTokensClaimable && (
                              <div className='item-price col-8'>
                                <strong>
                                  <small>{data.funding_token_id}</small>{' '}
                                  {prettyFormatAmount({
                                    value: unitPrice.toFixed(0),
                                    length: 50,
                                    showIsLessThanDecimalsLabel: false
                                  })}
                                </strong>
                                <span>/per unit</span>
                              </div>
                            )}

                            {!xProjectsToken?.includes(tokenId) && (
                              <div className='item-buttons col-4'>
                                <button
                                  className={`btn btn-${
                                    data.isTokensClaimable
                                      ? 'success'
                                      : 'warning'
                                  }`}
                                  onClick={() =>
                                    onBuyPropertyUnits({
                                      data
                                    })
                                  }
                                >
                                  {!data.isTokensClaimable
                                    ? 'Buy'
                                    : 'Claim Tokens'}
                                </button>
                              </div>
                            )}
                          </div>
                        </div>
                      </div>
                    </div>
                  );
                }
              )
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
