'use client';

import { XProjectsValue, useXProjects } from '@/contracts/xProject/hooks';
import { xProjectFundingSC } from '@/contracts/xProjectFunding';
import { getWindowLocation, prettyFormatAmount, sleep } from '@/utils';
import { RoutePath } from '@/utils/routes';
import { signAndSendTransactions } from '@/utils/signAndSendTransactions';
import { Transaction } from '@multiversx/sdk-core/out';
import { useGetAccount } from '@multiversx/sdk-dapp/hooks/account/useGetAccount';
import Link from 'next/link';
import { useCallback } from 'react';

export default function Properties() {
  // TODO set this from ref url
  const referrerID = undefined;
  const properties = useXProjects();

  const { address: loggedInUserAdsress } = useGetAccount();

  const onBuyPropertyUnits = useCallback(
    async ({ data }: Pick<XProjectsValue['projectData'], 'data'>) => {
      let transactions: Transaction[] = [];

      if (!data.isTokensClaimable) {
        const fundProjectTx = xProjectFundingSC.makeFundProjectTx({
          projectID: data.id,
          amount: data.funding_goal.dividedBy(3),
          referrerID
        });
        fundProjectTx.sender = loggedInUserAdsress;
        transactions.push(fundProjectTx);
      }
      if (data.isTokensClaimable) {
        const claimXProjectTokenTx = xProjectFundingSC.makeClaimXProjectTokenTx(
          {
            projectID: data.id
          }
        );

        claimXProjectTokenTx.sender = loggedInUserAdsress;
        transactions.push(claimXProjectTokenTx);
      }

      await signAndSendTransactions({
        transactions,
        callbackRoute: getWindowLocation().pathname,
        transactionsDisplayInfo: {}
      });
    },
    [loggedInUserAdsress]
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
                  let href = `${RoutePath.Properties}`;

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
                            <div className='feature'>{feature}</div>
                          ))}
                        </div>
                        <h3 className='item-title'>
                          <Link href={href}>{description}</Link>
                        </h3>

                        <div className='item-price-buttons row'>
                          <div className='col-12 row'>
                            <div className='item-price col-8'>
                              <strong>${rentPrice}</strong>
                              <span>/per year</span>
                            </div>
                            <div className='item-buttons col-4'>
                              <button className='btn btn-primary'>Rent</button>
                            </div>
                          </div>
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
                            <div className='item-buttons col-4'>
                              <button
                                className={`btn btn-${
                                  data.isTokensClaimable ? 'success' : 'warning'
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
